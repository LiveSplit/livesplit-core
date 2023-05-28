#![allow(clippy::unnecessary_cast)]

use std::{
    io,
    path::{self, Path},
    time::{Duration, Instant},
};

use proc_maps::{MapRange, Pid};
use read_process_memory::{CopyAddress, ProcessHandle};
use snafu::{OptionExt, ResultExt, Snafu};
use sysinfo::{self, PidExt, ProcessExt};

use crate::runtime::ProcessList;

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

pub struct Process {
    handle: ProcessHandle,
    pid: Pid,
    memory_ranges: Vec<MapRange>,
    last_check: Instant,
    path: Option<Box<str>>,
}

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid).finish()
    }
}

impl Process {
    pub fn with_name(name: &str, process_list: &mut ProcessList) -> Result<Self, OpenError> {
        process_list.refresh();
        let processes = process_list.processes_by_name(name);

        // Sorts the processes (asc) by numeric pid, to allow max_by_key to
        // select the higher pid in case all records are equally maximum; otherwise
        // use the process that was started the most recently, it's more
        // predictable for the user.

        let process = &processes
            .max_by_key(|p| (p.start_time(), p.pid().as_u32()))
            .context(ProcessDoesntExist)?;

        let path = build_path(process.exe());

        let pid = process.pid().as_u32() as Pid;

        let handle = pid.try_into().context(InvalidHandle)?;

        Ok(Process {
            handle,
            pid,
            memory_ranges: Vec::new(),
            last_check: Instant::now() - Duration::from_secs(1),
            path,
        })
    }

    pub fn is_open(&self, process_list: &mut ProcessList) -> bool {
        // FIXME: We can actually ask the list to only refresh the individual process.
        process_list.refresh();
        process_list.is_open(sysinfo::Pid::from_u32(self.pid as u32))
    }

    pub fn module_address(&mut self, module: &str) -> Result<Address, ModuleError> {
        self.refresh_memory_ranges()?;
        self.memory_ranges
            .iter()
            .find(|m| m.filename().map_or(false, |f| f.ends_with(module)))
            .context(ModuleDoesntExist)
            .map(|m| m.start() as u64)
    }

    pub fn module_size(&mut self, module: &str) -> Result<u64, ModuleError> {
        self.refresh_memory_ranges()?;
        Ok(self
            .memory_ranges
            .iter()
            .filter(|m| m.filename().map_or(false, |f| f.ends_with(module)))
            .map(|m| m.size() as u64)
            .sum())
    }

    pub fn read_mem(&self, address: Address, buf: &mut [u8]) -> io::Result<()> {
        self.handle.copy_address(address as usize, buf)
    }

    pub fn get_memory_range_count(&mut self) -> Result<usize, ModuleError> {
        self.refresh_memory_ranges()?;
        Ok(self.memory_ranges.len())
    }

    pub fn get_memory_range_address(&mut self, idx: usize) -> Result<Address, ModuleError> {
        self.memory_ranges
            .get(idx)
            .ok_or(ModuleError::ModuleDoesntExist)
            .map(|m| m.start() as Address)
    }

    pub fn get_memory_range_size(&mut self, idx: usize) -> Result<u64, ModuleError> {
        self.memory_ranges
            .get(idx)
            .ok_or(ModuleError::ModuleDoesntExist)
            .map(|m| m.size() as u64)
    }

    pub fn get_memory_range_flags(&mut self, idx: usize) -> Result<u64, ModuleError> {
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

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    fn refresh_memory_ranges(&mut self) -> Result<(), ModuleError> {
        let now = Instant::now();
        if now - self.last_check >= Duration::from_secs(1) {
            self.memory_ranges = match proc_maps::get_process_maps(self.pid) {
                Ok(m) => m,
                Err(source) => {
                    self.memory_ranges.clear();
                    return Err(ModuleError::ListModules { source });
                }
            };
            self.last_check = now;
        }
        Ok(())
    }
}

fn build_path(original_path: &Path) -> Option<Box<str>> {
    let mut path = String::from("/mnt");
    for component in original_path.components() {
        if !path.ends_with('/') {
            path.push('/');
        }
        match component {
            path::Component::Prefix(prefix) => match prefix.kind() {
                path::Prefix::VerbatimDisk(disk) | path::Prefix::Disk(disk) => {
                    path.push(disk.to_ascii_lowercase() as char)
                }
                _ => return None,
            },
            path::Component::Normal(c) => {
                path.push_str(c.to_str()?);
            }
            path::Component::RootDir => {}
            path::Component::CurDir => path.push('.'),
            path::Component::ParentDir => path.push_str(".."),
        }
    }
    Some(path.into_boxed_str())
}
