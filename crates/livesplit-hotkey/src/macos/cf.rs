#![allow(dead_code)]

use std::ffi::{c_ulong, c_void};

mod opaque {
    // TODO: Use extern types once they are stable.
    // Relevant issue about empty enums:
    // https://github.com/rust-lang/rust/issues/74840
    // Right now we do what rust-cpython does:
    // https://github.com/dgrunwald/rust-cpython/commit/06e61097a798bf9203884d3bb5ffddcf3296a3b2
    #[repr(C)]
    pub struct Allocator {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct MachPort {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct RunLoop {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct RunLoopSource {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct String {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct Data {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct Boolean {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct Dictionary {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct DictionaryKeyCallBacks {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct DictionaryValueCallBacks {
        _private: [u8; 0],
    }
}

pub type AllocatorRef = *mut opaque::Allocator;
pub type MachPortRef = *mut opaque::MachPort;
pub type RunLoopRef = *mut opaque::RunLoop;
pub type RunLoopSourceRef = *mut opaque::RunLoopSource;

pub type BooleanRef = *const opaque::Boolean;
pub type StringRef = *const opaque::String;
pub type TypeRef = *const c_void;
pub type DataRef = *const opaque::Data;
pub type DictionaryRef = *mut opaque::Dictionary;
pub type DictionaryKeyCallBacksRef = *const opaque::DictionaryKeyCallBacks;
pub type DictionaryValueCallBacksRef = *const opaque::DictionaryValueCallBacks;

pub type RunLoopMode = StringRef;

pub type Index = isize;

pub type OptionFlags = c_ulong;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct RunLoopActivity: OptionFlags {
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

    pub static kCFBooleanTrue: BooleanRef;

    pub static kCFRunLoopDefaultMode: RunLoopMode;

    pub static kCFTypeDictionaryKeyCallBacks: opaque::DictionaryKeyCallBacks;

    pub static kCFTypeDictionaryValueCallBacks: opaque::DictionaryValueCallBacks;

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

    pub fn CFDictionaryCreate(
        allocator: AllocatorRef,
        keys: *const TypeRef,
        values: *const TypeRef,
        numValues: Index,
        keyCallbacks: DictionaryKeyCallBacksRef,
        valueCallbacks: DictionaryValueCallBacksRef,
    ) -> DictionaryRef;
}
