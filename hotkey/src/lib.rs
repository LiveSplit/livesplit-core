#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(not(any(windows)))]
pub mod other;
#[cfg(not(any(windows)))]
pub use other::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KeyEvent {
    KeyUp(KeyCode),
    KeyDown(KeyCode),
}
