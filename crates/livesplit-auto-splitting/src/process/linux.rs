use libc::pid_t;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, Read};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

use nom::character::complete::{
    char, digit1, hex_digit1, newline, not_line_ending, one_of, space1,
};
use nom::combinator::{map, map_res, opt, verify};
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;

use super::{Address, Error, ProcessImpl, Result, ScannableRange};

#[derive(Debug)]
pub struct Process {
    pid: pid_t,
    is_64bit: bool,
    memory: RefCell<Option<File>>,
}

struct MapRange {
    range_start: Address,
    range_end: Address,
    offset: u64,
    path: Option<PathBuf>,
    perms: [u8; 4],
}

fn parse_hex(input: &[u8]) -> IResult<&[u8], u64> {
    map_res(hex_digit1, |input| {
        u64::from_str_radix(std::str::from_utf8(input).unwrap(), 16)
    })(input)
}

fn map_line(input: &[u8]) -> IResult<&[u8], MapRange> {
    map(
        tuple((
            terminated(separated_pair(parse_hex, char('-'), parse_hex), space1),
            terminated(
                tuple((one_of("r-"), one_of("w-"), one_of("x-"), one_of("sp"))),
                space1,
            ),
            terminated(parse_hex, space1),
            terminated(separated_pair(parse_hex, char(':'), parse_hex), space1),
            terminated(
                map_res(digit1, |input| {
                    u64::from_str_radix(std::str::from_utf8(input).unwrap(), 10)
                }),
                space1,
            ),
            opt(verify(
                map(not_line_ending, |input| {
                    PathBuf::from(OsStr::from_bytes(input).to_os_string())
                }),
                |i: &Path| !i.as_os_str().is_empty(),
            )),
            opt(newline),
        )),
        |((range_start, range_end), perms, offset, _dev, _inode, path, _newline)| MapRange {
            range_start,
            range_end,
            offset,
            path,
            perms: [perms.0 as u8, perms.1 as u8, perms.2 as u8, perms.3 as u8],
        },
    )(input)
}

struct MapRangeIter(std::io::Split<std::io::BufReader<File>>);

impl Iterator for MapRangeIter {
    type Item = MapRange;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(line)) => {
                    if let Ok((_, range)) = map_line(&line) {
                        return Some(range);
                    }
                }
                Some(Err(_)) => continue,
                None => return None,
            }
        }
    }
}

pub(crate) struct ScannableIter(MapRangeIter);

impl Iterator for ScannableIter {
    type Item = ScannableRange;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(range) = self.0.next() {
                if let Some(path) = range.path {
                    if path.starts_with("/dev")
                        || path.file_name().unwrap() == "[vvar]"
                        || path.file_name().unwrap() == "[vdso]"
                    {
                        continue;
                    }
                }

                if range.perms[0] == b'r' {
                    return Some(ScannableRange {
                        base: range.range_start,
                        len: range.range_end - range.range_start,
                    });
                }
            } else {
                return None;
            }
        }
    }
}

impl ProcessImpl for Process {
    fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    fn with_name(name: &OsStr) -> Result<Self> {
        let mut processes = Process::processes_with_name(name)?;
        if !processes.is_empty() {
            Ok(processes.swap_remove(0))
        } else {
            Err(Error::ProcessDoesntExist)
        }
    }

    fn module_address(&self, module: &OsStr) -> Result<Address> {
        self.modules()?
            .get(module)
            .cloned()
            .ok_or(Error::ModuleDoesntExist)
    }

    fn read_buf(&self, address: Address, buf: &mut [u8]) -> Result<()> {
        if let Some(file) = self.memory() {
            file.read_exact_at(buf, address as u64)
                .or(Err(Error::ReadMemory))
        } else {
            Err(Error::ReadMemory)
        }
    }

    type ScannableIter = ScannableIter;
    fn scannable_regions(&self) -> Result<Self::ScannableIter> {
        Ok(ScannableIter(self.memory_pages()?))
    }
}

impl Process {
    fn process_name(&self) -> Option<OsString> {
        let mut process_name = Vec::new();
        File::open(format!("/proc/{}/comm", self.pid))
            .ok()?
            .read_to_end(&mut process_name)
            .ok()?;

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
        let processes = dir
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_dir())
            .filter_map(|e| {
                e.path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .map(pid_t::from_str)
            })
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

    /*pub*/
    fn with_pid(pid: libc::pid_t) -> Result<Self> {
        let mut proc = Process {
            pid,
            is_64bit: false,
            memory: RefCell::new(None),
        };
        // TODO: do we want to cache the pages/modules at all?
        if let Ok(pages) = proc.memory_pages() {
            // Inspired by https://unix.stackexchange.com/a/106235 "Parsing the maps file"
            proc.is_64bit = pages.last().ok_or(Error::ProcessDoesntExist)?.range_end > 0xFFFFFFFF;
            Ok(proc)
        } else {
            Err(Error::ProcessDoesntExist)
        }
    }

    /*pub*/
    fn modules(&self) -> Result<HashMap<OsString, Address>> {
        // There are multiple ways of doing this. Could also traverse symlinks in /proc/PID/map_files, for instance
        Ok(self
            .memory_pages()?
            .filter_map(
                |MapRange {
                     path,
                     range_start,
                     offset,
                     ..
                 }| {
                    path.map(|p| {
                        (
                            p.file_name().unwrap().to_os_string(),
                            range_start - offset as Address,
                        )
                    })
                },
            )
            .collect())
    }

    fn memory(&self) -> Option<RefMut<File>> {
        let mut mem = self.memory.borrow_mut();
        if mem.is_none() {
            // TODO: alternative methods of obtaining file handles
            if let Ok(file) = File::open(format!("/proc/{}/mem", self.pid)) {
                *mem = Some(file);
            }
        }

        if mem.is_some() {
            Some(RefMut::map(mem, |o| o.as_mut().unwrap()))
        } else {
            None
        }
    }

    // Parses /proc/PID/maps
    fn memory_pages(&self) -> Result<MapRangeIter> {
        // License: MIT
        // Copyright (c) 2016 Julia Evans, Jorge Aparicio
        // Based on https://github.com/rbspy/proc-maps/blob/7168cd0a13d464ef00f20a81f064b7729ff58d2e/src/linux_maps.rs#L49
        let file = io::BufReader::new(
            File::open(format!("/proc/{}/maps", self.pid)).or(Err(Error::ProcessDoesntExist))?,
        );
        // TODO: use OsString, use unwrap a bit less.
        Ok(MapRangeIter(file.split(b'\n')))
    }
}
