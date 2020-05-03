use libc::pid_t;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::FileExt;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::{mem, slice, fs, io};
use std::io::{Read, BufRead};
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

use super::{Error, Result, Address, Offset, Signature, ProcessImpl};

#[derive(Debug)]
pub struct Process {
    pid: pid_t,
    is_64bit: bool,
    memory: RefCell<Option<File>>
}

struct MapRange {
    range_start: Address,
    range_end: Address,
    offset: Offset,
    path: Option<PathBuf>,
    perms: [u8; 4]
}

impl ProcessImpl for Process {
    fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    fn with_name(name: &OsStr) -> Result<Self> {
        let mut processes = Process::processes_with_name(name)?;
        if processes.len() >= 1 {
            Ok(processes.swap_remove(0))
        } else {
            Err(Error::ProcessDoesntExist)
        }
    }

    fn module_address(&self, module: &OsStr) -> Result<Address> {
        self.modules()?.get(module).cloned().ok_or(Error::ModuleDoesntExist)
    }

    fn read_buf(&self, address: Address, buf: &mut [u8]) -> Result<()> {
        use libc::{pid_t, c_void, iovec, process_vm_readv};

        if let Some(file) = self.memory() {
            file.read_exact_at(buf, address as u64).or(Err(Error::ReadMemory))
        } else {
            Err(Error::ReadMemory)
        }
    }

    // TODO: move to shared helper
    fn scan_signature(&self, signature: Signature) -> Result<Option<Address>> {
        let mut page_buf = Vec::<u8>::new();

        for page in self.memory_pages()?
            .filter(|range| range.perms[0] == b'r' && !(range.path.is_some() && (
                range.path.as_ref().unwrap().starts_with("/dev") ||
                range.path.as_ref().unwrap().file_name().unwrap() == "[vvar]" ||
                range.path.as_ref().unwrap().file_name().unwrap() == "[vdso]"
            )))
        {
            let base = page.range_start;
            let len = page.range_end - page.range_start;
            page_buf.clear();
            page_buf.reserve(len as usize);
            unsafe {
                page_buf.set_len(len as usize);
                self.read_buf(base, &mut page_buf)?;
            }
            if let Some(index) = signature.scan(&page_buf) {
                return Ok(Some(base + index as Address));
            }
        }
        Ok(None)
    }
}

impl Process {
    fn process_name(&self) -> Option<OsString> {
        let mut process_name = Vec::new();
        File::open(format!("/proc/{}/comm", self.pid)).ok()?.read_to_end(&mut process_name).ok()?;

        if let Some(last) = process_name.last() {
            if *last == b'\n' {
                let _ = last;
                let _ = process_name.pop();
            }
        }

        Some(OsString::from_vec(process_name))
    }

    // fn handle(&self) -> pid_t {
    //     self.pid
    // }

    // Autosplitter/user should probably get access to this list to choose from it in the future.
    fn processes_with_name(name: &OsStr) -> Result<Vec<Self>> {
        // License: MIT
        // Copyright (c) 2015 Guillaume Gomez
        // Based on https://github.com/GuillaumeGomez/sysinfo/blob/4edbf34ad5fcd03979498ec124e15a067c10d0b4/src/linux/system.rs#L512
        let dir = fs::read_dir("/proc").or(Err(Error::ListProcesses))?;
        let processes = dir.filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.path().file_name().and_then(OsStr::to_str).map(pid_t::from_str))
            .filter_map(std::result::Result::ok)
            .map(Process::with_pid)
            .filter_map(std::result::Result::ok)
            .filter(|proc| {
                if let Some(process_name) = proc.process_name() {
                    &*process_name == name
                } else {
                    false
                }
            })
            // TODO: sort
            .collect();

        Ok(processes)
    }

    /*pub*/ fn with_pid(pid: libc::pid_t) -> Result<Self> {
        let mut proc = Process { pid, is_64bit: false, memory: RefCell::new(None) };
        // TODO: do we want to cache the pages/modules at all?
        if let Ok(pages) = proc.memory_pages() {
            // Inspired by https://unix.stackexchange.com/a/106235 "Parsing the maps file"
            proc.is_64bit = pages.last().ok_or(Error::ProcessDoesntExist)?.range_end > 0xFFFFFFFF;
            Ok(proc)
        } else {
            Err(Error::ProcessDoesntExist)
        }
    }

    /*pub*/ fn modules(&self) -> Result<HashMap<OsString, Address>> {
        // There are multiple ways of doing this. Could also traverse symlinks in /proc/PID/map_files, for instance
        Ok(self.memory_pages()?.filter_map(|MapRange { path, range_start, offset, .. }|
            path.map(|p|
                (p.file_name().unwrap().to_os_string(), range_start - offset as Address)
            )
        ).collect())
    }

    fn memory(&self) -> Option<RefMut<File>> {
        let mut mem = self.memory.borrow_mut();
        if mem.is_none() {
            // TODO: alternative methods of obtaining file handles
            match File::open(format!("/proc/{}/mem", self.pid)) {
                Ok(file) => {
                    *mem = Some(file);
                }
                Err(_) => {} // do nothing
            }
        }

        if mem.is_some() {
            Some(RefMut::map(mem, |o| o.as_mut().unwrap()))
        } else {
            None
        }
    }

    // Parses /proc/PID/maps
    fn memory_pages(&self) -> Result<impl Iterator<Item=MapRange>> {
        // License: MIT
        // Copyright (c) 2016 Julia Evans, Jorge Aparicio
        // Based on https://github.com/rbspy/proc-maps/blob/7168cd0a13d464ef00f20a81f064b7729ff58d2e/src/linux_maps.rs#L49
        let file = io::BufReader::new(File::open(format!("/proc/{}/maps", self.pid)).or(Err(Error::ProcessDoesntExist))?);
        // TODO: use OsString, use unwrap a bit less.
        Ok(file.lines().filter_map(std::result::Result::ok).map(|line| {
            let mut fields = line.split_whitespace();

            let mut range = fields.next().unwrap().split('-');
            let range_start = Address::from_str_radix(range.next().unwrap(), 16).unwrap();
            let range_end = Address::from_str_radix(range.next().unwrap(), 16).unwrap();

            let perms = fields.next().unwrap().as_bytes();
            let perms: [u8; 4] = [perms[0], perms[1], perms[2], perms[3]];

            let offset = Offset::from_str_radix(fields.next().unwrap(), 16).unwrap();

            let _ = fields.next(); // dev
            let _ = fields.next(); // inode

            let path: Option<PathBuf> = fields.next().map(|s| s.into());

            MapRange {
                range_start,
                range_end,
                offset,
                perms,
                path
            }
        }))
    }
}
