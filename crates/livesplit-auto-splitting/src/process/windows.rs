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
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::{iter, mem, ptr, result, slice};

use super::{Address, Error, Offset, ProcessImpl, Result, ScannableRange, Signature};

#[derive(Debug)]
pub struct Process {
    handle: HANDLE,
    modules: HashMap<OsString, Address>,
    is_64bit: bool,
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

pub(crate) struct ScannableIter {
    handle: HANDLE,
    addr: u64,
    max: u64,
    all: bool,
}

impl Iterator for ScannableIter {
    type Item = ScannableRange;
    fn next(&mut self) -> Option<Self::Item> {
        const mbi_size: usize = mem::size_of::<MEMORY_BASIC_INFORMATION>();
        while self.addr < self.max {
            unsafe {
                let mut mbi: MEMORY_BASIC_INFORMATION = mem::uninitialized();
                if VirtualQueryEx(self.handle, self.addr as _, &mut mbi, mbi_size) == 0 {
                    break;
                }
                self.addr += mbi.RegionSize as u64;

                // We don't care about reserved / free pages
                if mbi.State != MEM_COMMIT {
                    continue;
                }

                // We can't read from guarded pages
                if !self.all && (mbi.Protect & PAGE_GUARD) != 0 {
                    continue;
                }

                // We can't read from no access pages
                if !self.all && (mbi.Protect & PAGE_NOACCESS) != 0 {
                    continue;
                }

                return Some(ScannableRange {
                    base: mbi.BaseAddress as u64,
                    len: mbi.RegionSize as u64,
                });
            }
        }
        None
    }
}

impl ProcessImpl for Process {
    fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    fn with_name(name: &OsStr) -> Result<Self> {
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

    fn module_address(&self, module: &OsStr) -> Result<Address> {
        self.modules
            .get(module)
            .cloned()
            .ok_or(Error::ModuleDoesntExist)
    }

    fn read_buf(&self, address: Address, buf: &mut [u8]) -> Result<()> {
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

    type ScannableIter = ScannableIter;
    fn scannable_regions(&self) -> Result<ScannableIter> {
        Ok(self.memory_pages(false))
    }
}

impl Process {
    /*pub*/
    fn path(&self) -> Option<PathBuf> {
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

    /*pub*/
    fn with_pid(pid: DWORD) -> Result<Self> {
        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false as _, pid);

            if !handle.is_null() {
                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);

                if snapshot == INVALID_HANDLE_VALUE {
                    CloseHandle(handle);
                    return Err(Error::ListModules);
                }

                // TODO: processes can dynamically load and unload processes...
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
                            let name = OsString::from_wide(name);
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

    /*pub*/
    fn modules(&self) -> Result<&HashMap<OsString, Address>> {
        // TODO: when do we want to refresh this?
        Ok(&self.modules)
    }

    fn memory_pages(&self, all: bool) -> ScannableIter {
        // hardcoded values because GetSystemInfo / GetNativeSystemInfo can't
        // return info for remote process
        let min = 0x10000u64;
        let max = if self.is_64bit() {
            0x00007FFFFFFEFFFFu64
        } else {
            0x7FFEFFFFu64
        };

        let mut addr = min;
        ScannableIter {
            handle: self.handle,
            addr: min,
            max,
            all,
        }
    }
}
