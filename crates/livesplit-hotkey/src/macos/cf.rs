use std::ffi::c_void;

mod opaque {
    pub enum Allocator {}
    pub enum MachPort {}
    pub enum RunLoop {}
    pub enum RunLoopSource {}
    pub enum String {}
    pub enum Data {}
}

pub type AllocatorRef = *mut opaque::Allocator;
pub type MachPortRef = *mut opaque::MachPort;
pub type RunLoopRef = *mut opaque::RunLoop;
pub type RunLoopSourceRef = *mut opaque::RunLoopSource;

pub type StringRef = *const opaque::String;
pub type TypeRef = *const c_void;
pub type DataRef = *const opaque::Data;

pub type RunLoopMode = StringRef;

pub type Index = isize;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct RunLoopActivity: usize {
        const RUN_LOOP_ENTRY = 1 << 0;
        const RUN_LOOP_BEFORE_TIMERS = 1 << 1;
        const RUN_LOOP_BEFORE_SOURCES = 1 << 2;
        const RUN_LOOP_BEFORE_WAITING = 1 << 5;
        const RUN_LOOP_AFTER_WAITING = 1 << 6;
        const RUN_LOOP_EXIT = 1 << 7;
        const RUN_LOOP_ALL_ACTIVITIES = 0x0FFFFFFF;
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub static kCFAllocatorDefault: AllocatorRef;

    pub static kCFRunLoopDefaultMode: RunLoopMode;

    pub fn CFMachPortCreateRunLoopSource(
        allocator: AllocatorRef,
        port: MachPortRef,
        order: Index,
    ) -> RunLoopSourceRef;

    pub fn CFRunLoopGetCurrent() -> RunLoopRef;

    pub fn CFRunLoopContainsSource(
        rl: RunLoopRef,
        source: RunLoopSourceRef,
        mode: RunLoopMode,
    ) -> bool;
    pub fn CFRunLoopAddSource(rl: RunLoopRef, source: RunLoopSourceRef, mode: RunLoopMode);
    pub fn CFRunLoopRemoveSource(rl: RunLoopRef, source: RunLoopSourceRef, mode: RunLoopMode);

    pub fn CFRunLoopRun();
    pub fn CFRunLoopStop(rl: RunLoopRef);

    pub fn CFRunLoopCopyCurrentMode(rl: RunLoopRef) -> RunLoopMode;

    pub fn CFRelease(cf: TypeRef);

    pub fn CFDataGetBytePtr(the_data: DataRef) -> *const u8;
}
