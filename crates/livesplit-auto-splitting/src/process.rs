#![allow(clippy::unnecessary_cast)]

use std::{
    io,
    time::{Duration, Instant},
};

use proc_maps::{MapRange, Pid};
use read_process_memory::{CopyAddress, ProcessHandle};
use snafu::{OptionExt, ResultExt, Snafu};
use sysinfo::{self, PidExt, ProcessExt};

use crate::{runtime::ProcessList, wasi_path};

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum OpenError {
    ProcessDoesntExist,
    InvalidHandle { source: io::Error },
}

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum ModuleError {
    ModuleDoesntExist,
    ListModules { source: io::Error },
}

pub type Address = u64;

// FIXME: Temporary workaround until this is merged and released:
// https://github.com/rbspy/read-process-memory/pull/21
struct UnsafeSendSync<T>(T);
// SAFETY: Temporary
unsafe impl<T> Send for UnsafeSendSync<T> {}
// SAFETY: Temporary
unsafe impl<T> Sync for UnsafeSendSync<T> {}

/// A process that an auto splitter is attached to.
pub struct Process {
    handle: UnsafeSendSync<ProcessHandle>,
    pid: Pid,
    memory_ranges: Vec<MapRange>,
    next_memory_range_check: Instant,
    next_open_check: Instant,
    path: Option<Box<str>>,
}

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process")
            .field("pid", &self.pid)
            .field("path", &self.path)
            .finish()
    }
}

impl Process {
    pub(super) fn with_name(name: &str, process_list: &mut ProcessList) -> Result<Self, OpenError> {
        process_list.refresh();
        let processes = process_list.processes_by_name(name);

        // Sorts the processes (asc) by numeric pid, to allow max_by_key to
        // select the higher pid in case all records are equally maximum; otherwise
        // use the process that was started the most recently, it's more
        // predictable for the user.

        let process = &processes
            .max_by_key(|p| (p.start_time(), p.pid().as_u32()))
            .context(ProcessDoesntExist)?;

        let path = wasi_path::from_native(process.exe());

        let pid = process.pid().as_u32() as Pid;

        let handle = UnsafeSendSync(pid.try_into().context(InvalidHandle)?);

        let now = Instant::now();
        Ok(Process {
            handle,
            pid,
            memory_ranges: Vec::new(),
            next_memory_range_check: now,
            next_open_check: now + Duration::from_secs(1),
            path,
        })
    }

    pub(super) fn with_pid(pid: u32, process_list: &mut ProcessList) -> Result<Self, OpenError> {
        process_list.refresh();
        let process = process_list
            .get(sysinfo::Pid::from_u32(pid))
            .context(ProcessDoesntExist)?;

        let path = wasi_path::from_native(process.exe());

        let pid_out = pid as Pid;

        let handle = UnsafeSendSync(pid_out.try_into().context(InvalidHandle)?);

        let now = Instant::now();
        Ok(Process {
            handle,
            pid: pid_out,
            memory_ranges: Vec::new(),
            next_memory_range_check: now,
            next_open_check: now + Duration::from_secs(1),
            path,
        })
    }

    pub(super) fn list_pids_by_name<'a>(
        name: &'a str,
        process_list: &'a mut ProcessList,
    ) -> impl Iterator<Item = u32> + 'a {
        process_list.refresh();
        process_list
            .processes_by_name(name)
            .map(|p| p.pid().as_u32())
    }

    pub(super) fn is_open(&mut self, process_list: &mut ProcessList) -> bool {
        let now = Instant::now();
        let pid = sysinfo::Pid::from_u32(self.pid as u32);
        if now >= self.next_open_check {
            process_list.refresh_single_process(pid);
            self.next_open_check = now + Duration::from_secs(1);
        }
        process_list.is_open(pid)
    }

    pub(super) fn module_address(&mut self, module: &str) -> Result<Address, ModuleError> {
        self.refresh_memory_ranges()?;
        self.memory_ranges
            .iter()
            .find(|m| m.filename().is_some_and(|f| f.ends_with(module)))
            .context(ModuleDoesntExist)
            .map(|m| m.start() as u64)
    }

    pub(super) fn module_size(&mut self, module: &str) -> Result<u64, ModuleError> {
        self.refresh_memory_ranges()?;
        Ok(self
            .memory_ranges
            .iter()
            .filter(|m| m.filename().is_some_and(|f| f.ends_with(module)))
            .map(|m| m.size() as u64)
            .sum())
    }

    pub(super) fn module_path(&mut self, module: &str) -> Result<Box<str>, ModuleError> {
        self.refresh_memory_ranges()?;
        self.memory_ranges
            .iter()
            .find(|m| m.filename().is_some_and(|f| f.ends_with(module)))
            .context(ModuleDoesntExist)
            .map(|m| wasi_path::from_native(m.filename().unwrap()).unwrap_or_default())
    }

    pub(super) fn read_mem(&self, address: Address, buf: &mut [u8]) -> io::Result<()> {
        self.handle.0.copy_address(address as usize, buf)
    }

    pub(super) fn get_memory_range_count(&mut self) -> Result<usize, ModuleError> {
        self.refresh_memory_ranges()?;
        Ok(self.memory_ranges.len())
    }

    pub(super) fn get_memory_range_address(&mut self, idx: usize) -> Result<Address, ModuleError> {
        self.memory_ranges
            .get(idx)
            .ok_or(ModuleError::ModuleDoesntExist)
            .map(|m| m.start() as Address)
    }

    pub(super) fn get_memory_range_size(&mut self, idx: usize) -> Result<u64, ModuleError> {
        self.memory_ranges
            .get(idx)
            .ok_or(ModuleError::ModuleDoesntExist)
            .map(|m| m.size() as u64)
    }

    pub(super) fn get_memory_range_flags(&mut self, idx: usize) -> Result<u64, ModuleError> {
        let module = self
            .memory_ranges
            .get(idx)
            .ok_or(ModuleError::ModuleDoesntExist)?;

        // We start with a non-zero flag, because we consider 0 to be an invalid flag.
        let mut flags = 1;
        if module.is_read() {
            flags |= 1 << 1;
        }
        if module.is_write() {
            flags |= 1 << 2;
        }
        if module.is_exec() {
            flags |= 1 << 3;
        }
        if module.filename().is_some() {
            flags |= 1 << 4;
        }

        Ok(flags)
    }

    /// Returns the process id of the process.
    pub const fn pid(&self) -> Pid {
        self.pid
    }

    /// Returns the path of the executable of the process.
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns the name of the executable of the process.
    pub fn name(&self) -> Option<&str> {
        let path = &self.path.as_deref()?;
        Some(match path.rsplit_once('/') {
            Some((_, name)) => name,
            None => path,
        })
    }

    fn refresh_memory_ranges(&mut self) -> Result<(), ModuleError> {
        let now = Instant::now();
        if now >= self.next_memory_range_check {
            self.memory_ranges = match proc_maps::get_process_maps(self.pid) {
                Ok(m) => m,
                Err(source) => {
                    self.memory_ranges.clear();
                    return Err(ModuleError::ListModules { source });
                }
            };
            self.next_memory_range_check = now + Duration::from_secs(1);
        }
        Ok(())
    }
}
