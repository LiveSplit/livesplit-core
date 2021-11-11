use read_process_memory::{CopyAddress, ProcessHandle};
use std::cell::RefCell;
use sysinfo::{self, AsU32, ProcessExt, RefreshKind, System, SystemExt};

thread_local! {
    static SYSTEM: RefCell<System> =
        RefCell::new(System::new_with_specifics(RefreshKind::new().with_processes()));
}

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

#[cfg(target_os = "windows")]
type OurPid = u32;
#[cfg(not(target_os = "windows"))]
type OurPid = i32;

pub struct Process {
    handle: ProcessHandle,
    pid: OurPid,
}

impl Process {
    pub fn with_name(name: &str) -> Result<Self> {
        let pid = SYSTEM.with(|s| {
            let mut sys = s.borrow_mut();
            sys.refresh_processes();
            let processes = sys.process_by_name(name);
            processes
                .first()
                .ok_or(Error::ProcessDoesntExist)
                .map(|&p| p.pid().as_u32() as OurPid)
        })?;
        Ok(Process {
            handle: pid.try_into().map_err(|_| Error::ProcessOpening)?,
            pid,
        })
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
