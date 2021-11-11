use proc_maps::Pid;
use read_process_memory::{CopyAddress, ProcessHandle};
use sysinfo::{self, ProcessExt, System, SystemExt};

#[derive(Debug)]
pub enum Error {
    // TODO: Doc Comments
    ProcessDoesntExist,
    ListModules,
    ProcessOpening,
    ModuleDoesntExist,
    ReadMemory,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type Address = u64;

pub struct Process {
    handle: ProcessHandle,
    pid: Pid,
}

impl Process {
    pub fn with_name(name: &str, sysinfo: &mut System) -> Result<Self> {
        sysinfo.refresh_processes();
        let processes = sysinfo.process_by_name(name);
        let pid = processes
            .first()
            .ok_or(Error::ProcessDoesntExist)
            .map(|&p| p.pid())? as Pid;
        let handle = pid.try_into().map_err(|_| Error::ProcessOpening)?;
        Ok(Process { handle, pid })
    }

    pub fn module_address(&self, module: &str) -> Result<Address> {
        if let Ok(maps) = proc_maps::get_process_maps(self.pid) {
            maps.iter()
                .find(|m| {
                    m.filename()
                        .as_ref()
                        .map(|f| f.contains(module))
                        .unwrap_or_default()
                })
                .map(|m| m.start() as u64)
                .ok_or(Error::ModuleDoesntExist)
        } else {
            Err(Error::ListModules)
        }
    }

    pub fn read_mem(&self, address: Address, buf: &mut [u8]) -> Result<()> {
        self.handle
            .copy_address(address as usize, buf)
            .or(Err(Error::ReadMemory))
    }
}
