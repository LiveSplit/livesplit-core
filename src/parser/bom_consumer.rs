use std::io::{Read, Result};

#[derive(Copy, Clone)]
enum BomConsumerState {
    NotStarted,
    RemainingBytes([u8; 3], usize),
    Done,
}

pub struct BomConsumer<R>(R, BomConsumerState);

impl<R> From<R> for BomConsumer<R> {
    fn from(reader: R) -> Self {
        BomConsumer(reader, BomConsumerState::NotStarted)
    }
}

impl<R: Read> Read for BomConsumer<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        use self::BomConsumerState::*;

        let mut buf_offset = 0;

        loop {
            self.1 = match self.1 {
                NotStarted => {
                    let mut bytes = [0; 3];
                    self.0.read_exact(&mut bytes)?;
                    if bytes == [0xEF, 0xBB, 0xBF] {
                        Done
                    } else {
                        RemainingBytes(bytes, 0)
                    }
                }
                RemainingBytes(bytes, offset) => {
                    for (s, d) in bytes[offset..].iter().zip(buf[buf_offset..].iter_mut()) {
                        *d = *s;
                    }
                    let copy_count = 3 - offset;
                    if copy_count <= buf.len() {
                        buf_offset += copy_count;
                        Done
                    } else {
                        self.1 = RemainingBytes(bytes, offset + buf.len());
                        return Ok(buf.len());
                    }
                }
                Done => {
                    return Ok(self.0.read(&mut buf[buf_offset..])? + buf_offset);
                }
            };
        }
    }
}
