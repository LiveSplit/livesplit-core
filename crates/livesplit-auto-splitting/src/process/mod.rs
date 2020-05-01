#[cfg_attr(linux, path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod os;

pub use os::Process;

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
