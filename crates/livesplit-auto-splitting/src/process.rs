use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::{
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    memoryapi::{ReadProcessMemory, VirtualQueryEx},
    processthreadsapi::{GetProcessTimes, OpenProcess},
    psapi::GetModuleFileNameExW,
    tlhelp32::{
        CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, Process32FirstW, Process32NextW,
        MODULEENTRY32W, PROCESSENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
    },
    winnt::{
        HANDLE, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS,
        PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
    },
};

use std::collections::HashMap;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::{iter, mem, ptr, result, slice};

type Address = u64;
pub type Offset = i64;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    // TODO: Doc Comments
    ListProcesses,
    ProcessDoesntExist,
    ListModules,
    ProcessOpening,
    ModuleDoesntExist,
    ReadMemory,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Process {
    handle: HANDLE,
    modules: HashMap<String, Address>,
    is_64bit: bool,
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

impl Process {
    pub fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    pub fn path(&self) -> Option<PathBuf> {
        let mut path_buf = [0u16; 1024];
        if unsafe {
            GetModuleFileNameExW(
                self.handle,
                ptr::null_mut(),
                path_buf.as_mut_ptr() as *mut _,
                path_buf.len() as _,
            )
        } == 0
        {
            return None;
        }
        Some(PathBuf::from(OsString::from_wide(&path_buf)))
    }

    pub fn with_name(name: &str) -> Result<Self> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

            if snapshot == INVALID_HANDLE_VALUE {
                return Err(Error::ListProcesses);
            }

            let mut creation_time = mem::uninitialized();
            let mut exit_time = mem::uninitialized();
            let mut kernel_time = mem::uninitialized();
            let mut user_time = mem::uninitialized();

            let mut best_process = None::<(DWORD, u64)>;
            let mut entry: PROCESSENTRY32W = mem::uninitialized();
            entry.dwSize = mem::size_of_val(&entry) as _;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    {
                        let entry_name = &entry.szExeFile;
                        let len = entry_name.iter().take_while(|&&c| c != 0).count();
                        let entry_name = &entry_name[..len];
                        let entry_name = &OsString::from_wide(entry_name);
                        if entry_name == name {
                            let pid = entry.th32ProcessID;
                            let process = OpenProcess(PROCESS_QUERY_INFORMATION, false as _, pid);

                            if !process.is_null() {
                                let success = GetProcessTimes(
                                    process,
                                    &mut creation_time,
                                    &mut exit_time,
                                    &mut kernel_time,
                                    &mut user_time,
                                );
                                if success != 0 {
                                    let time = (creation_time.dwHighDateTime as u64) << 32
                                        | (creation_time.dwLowDateTime as u64);

                                    if best_process.map_or(true, |(_, oldest)| time > oldest) {
                                        best_process = Some((pid, time));
                                    }
                                }

                                CloseHandle(process);
                            }
                        }
                    }

                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);

            if let Some((pid, _)) = best_process {
                Process::with_pid(pid)
            } else {
                Err(Error::ProcessDoesntExist)
            }
        }
    }

    pub fn with_pid(pid: DWORD) -> Result<Self> {
        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false as _, pid);

            if !handle.is_null() {
                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);

                if snapshot == INVALID_HANDLE_VALUE {
                    CloseHandle(handle);
                    return Err(Error::ListModules);
                }

                let mut modules = HashMap::new();
                let mut entry: MODULEENTRY32W = mem::uninitialized();
                entry.dwSize = mem::size_of_val(&entry) as _;

                if Module32FirstW(snapshot, &mut entry) != 0 {
                    loop {
                        {
                            let base_address = entry.modBaseAddr as Address;
                            let name = &entry.szModule;
                            let len = name.iter().take_while(|&&c| c != 0).count();
                            let name = &name[..len];
                            let name = OsString::from_wide(name).to_string_lossy().into_owned();
                            modules.insert(name, base_address);
                        }

                        if Module32NextW(snapshot, &mut entry) == 0 {
                            break;
                        }
                    }
                }

                let is_64bit;
                #[cfg(target_pointer_width = "64")]
                {
                    use winapi::um::wow64apiset::IsWow64Process;

                    let mut pbool: BOOL = 0;
                    IsWow64Process(handle, &mut pbool);
                    is_64bit = pbool == 0;
                }
                #[cfg(not(target_pointer_width = "64"))]
                {
                    // TODO: Actually idk if 32-bit apps can read from 64-bit
                    // apps. If they can, then this is wrong.
                    is_64bit = false;
                }

                CloseHandle(snapshot);

                Ok(Self {
                    handle,
                    modules,
                    is_64bit,
                })
            } else {
                Err(Error::ProcessOpening)
            }
        }
    }

    pub fn module_address(&self, module: &str) -> Result<Address> {
        self.modules
            .get(module)
            .cloned()
            .ok_or(Error::ModuleDoesntExist)
    }

    pub fn read_buf(&self, address: Address, buf: &mut [u8]) -> Result<()> {
        unsafe {
            let mut bytes_read = mem::uninitialized();

            let successful = ReadProcessMemory(
                self.handle,
                address as _,
                buf.as_mut_ptr() as _,
                buf.len() as _,
                &mut bytes_read,
            ) != 0;

            if successful && bytes_read as usize == buf.len() {
                Ok(())
            } else {
                Err(Error::ReadMemory)
            }
        }
    }

    pub fn read<T: Copy>(&self, address: Address) -> Result<T> {
        // TODO Unsound af
        unsafe {
            let mut res = mem::uninitialized();
            let buf = slice::from_raw_parts_mut(mem::transmute(&mut res), mem::size_of::<T>());
            self.read_buf(address, buf).map(|_| res)
        }
    }

    fn memory_pages(&self, all: bool) -> impl Iterator<Item = MEMORY_BASIC_INFORMATION> + '_ {
        // hardcoded values because GetSystemInfo / GetNativeSystemInfo can't
        // return info for remote process
        let min = 0x10000u64;
        let max = if self.is_64bit() {
            0x00007FFFFFFEFFFFu64
        } else {
            0x7FFEFFFFu64
        };

        let mbi_size = mem::size_of::<MEMORY_BASIC_INFORMATION>();
        let mut addr = min;
        iter::from_fn(move || {
            while addr < max {
                unsafe {
                    let mut mbi: MEMORY_BASIC_INFORMATION = mem::uninitialized();
                    if VirtualQueryEx(self.handle, addr as _, &mut mbi, mbi_size) == 0 {
                        break;
                    }
                    addr += mbi.RegionSize as u64;

                    // We don't care about reserved / free pages
                    if mbi.State != MEM_COMMIT {
                        continue;
                    }

                    // We can't read from guarded pages
                    if !all && (mbi.Protect & PAGE_GUARD) != 0 {
                        continue;
                    }

                    // We can't read from no access pages
                    if !all && (mbi.Protect & PAGE_NOACCESS) != 0 {
                        continue;
                    }

                    return Some(mbi);
                }
            }
            None
        })
    }

    pub fn scan_signature(&self, signature: &str) -> Result<Option<Address>> {
        let signature = Signature::new(signature);

        let mut page_buf = Vec::<u8>::new();

        for page in self.memory_pages(false) {
            let base = page.BaseAddress as Address;
            let len = page.RegionSize as usize;
            page_buf.clear();
            page_buf.reserve(len);
            unsafe {
                page_buf.set_len(len);
            }
            self.read_buf(base, &mut page_buf)?;
            if let Some(index) = signature.scan(&page_buf) {
                return Ok(Some(base + index as Address));
            }
        }
        Ok(None)
    }
}

struct Signature {
    bytes: Vec<u8>,
    mask: Vec<bool>,
    skip_offsets: [usize; 256],
}

impl Signature {
    fn new(signature: &str) -> Self {
        let mut bytes_iter = signature.bytes().filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 0xA),
            b'A'..=b'F' => Some(b - b'A' + 0xA),
            b'?' => Some(0x10),
            _ => None,
        });
        let (mut bytes, mut mask) = (Vec::new(), Vec::new());

        while let (Some(a), Some(b)) = (bytes_iter.next(), bytes_iter.next()) {
            let sig_byte = (a << 4) | b;
            let is_question_marks = a == 0x10 && b == 0x10;
            bytes.push(sig_byte);
            mask.push(is_question_marks);
        }

        let mut skip_offsets = [0; 256];

        let mut unknown = 0;
        let end = bytes.len() - 1;
        for (i, (&byte, mask)) in bytes.iter().zip(&mask).enumerate().take(end) {
            if !mask {
                skip_offsets[byte as usize] = end - i;
            } else {
                unknown = end - i;
            }
        }

        if unknown == 0 {
            unknown = bytes.len();
        }

        for offset in &mut skip_offsets[..] {
            if unknown < *offset || *offset == 0 {
                *offset = unknown;
            }
        }

        Self {
            bytes,
            mask,
            skip_offsets,
        }
    }

    fn scan(&self, buf: &[u8]) -> Option<usize> {
        let mut current = 0;
        let end = self.bytes.len() - 1;
        while current <= buf.len() - self.bytes.len() {
            let rem = &buf[current..];
            if rem
                .iter()
                .zip(&self.bytes)
                .zip(&self.mask)
                .all(|((&buf, &search), &mask)| buf == search || mask)
            {
                return Some(current);
            }
            let offset = self.skip_offsets[rem[end] as usize];
            current += offset;
        }
        None
    }
}
