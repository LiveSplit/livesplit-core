use bstr::ByteSlice;
use log::Level;
use std::io::{self, IoSlice, IoSliceMut, LineWriter, Write};
use wasi::types::Filesize;
use wasi_common::{wasi, FileContents};

struct Log(Level);

impl Write for Log {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        log::log!(target: "Auto Splitter", self.0, "{}", buf.trim_end().as_bstr());
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct StdStream(LineWriter<Log>);

impl StdStream {
    pub fn stdout() -> Self {
        Self(LineWriter::new(Log(Level::Info)))
    }
    pub fn stderr() -> Self {
        Self(LineWriter::new(Log(Level::Error)))
    }
}

impl FileContents for StdStream {
    fn max_size(&self) -> Filesize {
        usize::MAX as Filesize
    }
    fn size(&self) -> Filesize {
        0
    }
    fn resize(&mut self, _new_size: Filesize) -> wasi::Result<()> {
        Ok(())
    }
    fn pwritev(&mut self, iovs: &[IoSlice], _offset: Filesize) -> wasi::Result<usize> {
        Ok(self.0.write_vectored(iovs)?)
    }
    fn preadv(&self, _iovs: &mut [IoSliceMut], _offset: Filesize) -> wasi::Result<usize> {
        Ok(0)
    }
    fn pwrite(&mut self, buf: &[u8], _offset: Filesize) -> wasi::Result<usize> {
        Ok(self.0.write(buf)?)
    }
    fn pread(&self, _buf: &mut [u8], _offset: Filesize) -> wasi::Result<usize> {
        Ok(0)
    }
}
