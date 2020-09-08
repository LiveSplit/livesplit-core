#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod os;

pub use os::Process;

use bytemuck::Pod;
use std::ffi::OsStr;
use std::mem;

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

pub type Result<T> = std::result::Result<T, Error>;

pub type Address = u64;
pub type Offset = i64;

pub(crate) struct Signature {
    bytes: Vec<u8>,
    mask: Vec<bool>,
    skip_offsets: [usize; 256],
}

impl Signature {
    pub(crate) fn new(signature: &str) -> Self {
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

    pub(crate) fn scan(&self, buf: &[u8]) -> Option<usize> {
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

pub(crate) struct ScannableRange {
    base: Address,
    len: u64,
}

/// Private trait used for keeping API consistent between platforms
trait ProcessImpl {
    fn is_64bit(&self) -> bool;
    fn with_name(name: &OsStr) -> Result<Self>
    where
        Self: Sized;
    //fn with_pid(pid: pid_t) -> Result<Self>;
    fn module_address(&self, module: &OsStr) -> Result<Address>;
    fn read_buf(&self, address: Address, buf: &mut [u8]) -> Result<()>;

    type ScannableIter;
    fn scannable_regions(&self) -> Result<Self::ScannableIter>;
}

impl Process {
    /// Returns whether this Process is 64 bit or not
    #[inline]
    pub fn is_64bit(&self) -> bool {
        ProcessImpl::is_64bit(self)
    }

    /// Finds a process with a given name and returns it
    #[inline]
    pub fn with_name<T: AsRef<OsStr>>(name: T) -> Result<Self> {
        ProcessImpl::with_name(name.as_ref())
    }

    /// Returns the address of a module within this process, if present
    #[inline]
    pub fn module_address<T: AsRef<OsStr>>(&self, module: T) -> Result<Address> {
        ProcessImpl::module_address(self, module.as_ref())
    }

    /// Reads bef.len() bytes from address in this process into buf
    #[inline]
    pub fn read_buf<T: AsMut<[u8]>>(&self, address: Address, mut buf: T) -> Result<()> {
        ProcessImpl::read_buf(self, address, buf.as_mut())
    }

    /// Reads a T from address in this process
    pub fn read<T: Pod>(&self, address: Address) -> Result<T> {
        // TODO: for some reason, we're unable to allocate this on the stack?
        // "`std::marker::Sized` is not implemented for `T`" even if I add it as an explicit bound
        let mut buf = vec![0; mem::size_of::<T>()];
        self.read_buf(address, &mut buf)?;
        Ok(*bytemuck::try_from_bytes_mut(&mut buf).or(Err(Error::ReadMemory))?)
    }

    fn scan_inner(&self, signature: Signature) -> Result<Option<Address>> {
        let mut page_buf = Vec::<u8>::new();

        for page in self.scannable_regions()? {
            let base = page.base;
            let len = page.len;
            page_buf.clear();
            page_buf.reserve(len as usize);
            unsafe {
                page_buf.set_len(len as usize);
                // TODO: Handle an error in reading memory more gracefully instead of silently
                // returning an array of 0s.
                match self.read_buf(base, &mut page_buf) {
                    Ok(_) => (),
                    Err(_) => {
                        for el in &mut page_buf {
                            *el = 0;
                        }
                    }
                };
            }
            if let Some(index) = signature.scan(&page_buf) {
                return Ok(Some(base + index as Address));
            }
        }
        Ok(None)
    }

    /// Returns the address of a matching signature, if present.
    pub fn scan_signature<T: AsRef<str>>(&self, signature: T) -> Result<Option<Address>> {
        self.scan_inner(Signature::new(signature.as_ref()))
    }
}
