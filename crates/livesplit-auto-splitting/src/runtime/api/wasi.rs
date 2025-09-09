use std::{
    collections::VecDeque,
    path::Path,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{self, AtomicUsize},
    },
    task::{Context, Poll},
};

use bstr::ByteSlice;
use tokio::io::{self, AsyncWrite};
use wasmtime_wasi::{
    DirPerms, FilePerms, WasiCtxBuilder,
    cli::{IsTerminal, StdoutStream},
    p2::{OutputStream, Pollable, StreamError},
    preview1::WasiP1Ctx,
};

use crate::{Timer, wasi_path};

const ERR_CAPACITY: usize = 1 << 20;

#[derive(Clone)]
pub struct StdErr {
    buffer: Arc<Buf>,
}

struct Buf {
    flush_idx: AtomicUsize,
    buf: Mutex<VecDeque<u8>>,
}

impl StdoutStream for StdErr {
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(self.clone())
    }

    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(self.clone())
    }
}

impl IsTerminal for StdErr {
    fn is_terminal(&self) -> bool {
        false
    }
}

impl StdErr {
    pub fn new() -> Self {
        StdErr {
            buffer: Arc::new(Buf {
                flush_idx: AtomicUsize::new(0),
                buf: Mutex::new(VecDeque::new()),
            }),
        }
    }

    pub fn print_lines<T: Timer>(&self, timer: &mut T) {
        let flush_idx = self.buffer.flush_idx.swap(0, atomic::Ordering::Relaxed);
        if flush_idx == 0 {
            return;
        }
        let buf = &mut *self.buffer.buf.lock().unwrap();
        let (first, _) = buf.as_slices();
        let to_print = match first.get(..flush_idx) {
            Some(to_print) => to_print,
            None => &buf.make_contiguous()[..flush_idx],
        };
        timer.log_auto_splitter(format_args!("{}", to_print.trim().as_bstr()));
        buf.drain(..flush_idx);
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), StreamError> {
        let buffer = &mut *self.buffer.buf.lock().unwrap();
        if bytes.len() > ERR_CAPACITY - buffer.len() {
            return Err(StreamError::Trap(anyhow::format_err!(
                "write beyond capacity of StdErr"
            )));
        }

        self.buffer.flush_idx.store(
            buffer.len() + bytes.iter().rposition(|&b| b == b'\n').unwrap_or_default(),
            atomic::Ordering::Relaxed,
        );

        buffer.extend(bytes);
        Ok(())
    }
}

impl OutputStream for StdErr {
    fn write(&mut self, bytes: bytes::Bytes) -> Result<(), StreamError> {
        self.write(&bytes)
    }

    fn flush(&mut self) -> Result<(), StreamError> {
        let len = self.buffer.buf.lock().unwrap().len();
        self.buffer.flush_idx.store(len, atomic::Ordering::Relaxed);
        Ok(())
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        let consumed = self.buffer.buf.lock().unwrap().len();
        Ok(ERR_CAPACITY.saturating_sub(consumed))
    }
}

impl AsyncWrite for StdErr {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.as_mut().write(buf) {
            Ok(()) => Poll::Ready(Ok(buf.len())),
            Err(e) => Poll::Ready(Err(io::Error::other(e))),
        }
    }
    fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(self.as_mut().flush().map_err(io::Error::other))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[async_trait::async_trait]
impl Pollable for StdErr {
    async fn ready(&mut self) {}
}

pub fn build(script_path: Option<&Path>) -> (WasiP1Ctx, StdErr) {
    let mut wasi = WasiCtxBuilder::new();
    let stderr = StdErr::new();
    wasi.stderr(stderr.clone());

    if let Some(script_path) = script_path
        && let Some(path) = wasi_path::from_native(script_path)
    {
        wasi.env("SCRIPT_PATH", &path);
    }

    #[cfg(windows)]
    {
        // SAFETY: This is always safe to call.
        let mut drives = unsafe { windows_sys::Win32::Storage::FileSystem::GetLogicalDrives() };
        loop {
            let drive_idx = drives.trailing_zeros();
            if drive_idx >= 26 {
                break;
            }
            drives &= !(1 << drive_idx);
            let drive = drive_idx as u8 + b'a';
            // Unfortunate if this fails, but we should still continue.
            let _ = wasi.preopened_dir(
                std::str::from_utf8(&[b'\\', b'\\', b'?', b'\\', drive, b':', b'\\']).unwrap(),
                std::str::from_utf8(&[b'/', b'm', b'n', b't', b'/', drive]).unwrap(),
                DirPerms::READ,
                FilePerms::READ,
            );
        }

        // FIXME: Unfortunately wasmtime doesn't support us defining our own
        // file system logic anymore.

        // wasi.push_dir(Box::new(DeviceDir), PathBuf::from("/mnt/device"))
        //     .unwrap();
    }
    #[cfg(not(windows))]
    {
        // Unfortunate if this fails, but we should still continue.
        let _ = wasi.preopened_dir("/", "/mnt", DirPerms::READ, FilePerms::READ);
    }
    (wasi.build_p1(), stderr)
}
