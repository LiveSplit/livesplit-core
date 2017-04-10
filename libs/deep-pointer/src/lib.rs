extern crate winapi;
extern crate kernel32;
#[macro_use]
extern crate quick_error;

mod process;

pub use process::{Process, Error, Result};

pub type Address = u64;
pub type Offset = i64;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DeepPointer {
    pub module: String,
    pub offsets: Vec<Offset>,
}

impl DeepPointer {
    pub fn new<S>(module: S, offsets: Vec<Offset>) -> Self
        where S: Into<String>
    {
        Self {
            module: module.into(),
            offsets: offsets,
        }
    }

    pub fn deref<T: Copy>(&self, process: &Process) -> Result<T> {
        let mut address = process.module_address(&self.module)?;
        let mut offsets = self.offsets.iter().cloned();
        if let Some(mut offset) = offsets.next() {
            loop {
                address = (address as Offset).wrapping_add(offset) as Address;

                if let Some(new_offset) = offsets.next() {
                    offset = new_offset;
                    address = process.read(address)?;
                } else {
                    break;
                }
            }
        }
        process.read(address)
    }
}

#[test]
fn test() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let process = Process::with_name("notepad.exe").unwrap();
    let ptr = DeepPointer::new("tiptsf.dll", vec![0x7A000, 0x4c0, 0x2b8]);
    let text: [u16; 13] = ptr.deref(&process).unwrap();
    let text = OsString::from_wide(&text);
    let text = text.to_string_lossy().to_owned();
    assert_eq!("meh okay, idk", text);
}
