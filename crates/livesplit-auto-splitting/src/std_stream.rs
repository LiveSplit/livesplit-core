use bstr::ByteSlice;
use log::Level;
use wasi_common::{WasiFile, pipe::WritePipe};
use std::io::{self, LineWriter, Write};

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

pub fn stdout() -> impl WasiFile {
    WritePipe::new(LineWriter::new(Log(Level::Info)))
}

pub fn stderr() -> impl WasiFile {
    WritePipe::new(LineWriter::new(Log(Level::Error)))
}
