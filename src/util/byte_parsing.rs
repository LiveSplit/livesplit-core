use core::mem;

use bytemuck::AnyBitPattern;

pub mod big_endian {
    use super::strip_pod;
    use bytemuck::{Pod, Zeroable};
    use core::fmt;

    #[derive(Copy, Clone, Pod, Zeroable)]
    #[repr(transparent)]
    pub struct U16(pub [u8; 2]);

    impl fmt::Debug for U16 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.get(), f)
        }
    }

    impl U16 {
        pub const fn get(self) -> u16 {
            u16::from_be_bytes(self.0)
        }

        #[cfg(any(windows, feature = "default-text-engine"))]
        pub const fn usize(self) -> usize {
            self.get() as usize
        }
    }

    #[derive(Copy, Clone, Pod, Zeroable)]
    #[repr(transparent)]
    pub struct U32(pub [u8; 4]);

    impl fmt::Debug for U32 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.get(), f)
        }
    }

    impl U32 {
        pub const fn get(self) -> u32 {
            u32::from_be_bytes(self.0)
        }

        #[cfg(feature = "default-text-engine")]
        pub const fn usize(self) -> usize {
            self.get() as usize
        }
    }

    pub fn strip_u16(cursor: &mut &[u8]) -> Option<u16> {
        Some(u16::from_be_bytes(*strip_pod(cursor)?))
    }

    pub fn strip_u32(cursor: &mut &[u8]) -> Option<u32> {
        Some(u32::from_be_bytes(*strip_pod(cursor)?))
    }

    pub fn strip_u64(cursor: &mut &[u8]) -> Option<u64> {
        Some(u64::from_be_bytes(*strip_pod(cursor)?))
    }
}

pub fn strip_u8(cursor: &mut &[u8]) -> Option<u8> {
    strip_pod(cursor).copied()
}

pub fn strip_pod<'a, T: AnyBitPattern>(cursor: &mut &'a [u8]) -> Option<&'a T> {
    if cursor.len() < mem::size_of::<T>() {
        return None;
    }
    let (before, after) = cursor.split_at(mem::size_of::<T>());
    *cursor = after;
    Some(bytemuck::from_bytes(before))
}

pub fn strip_slice<'a, T: AnyBitPattern>(cursor: &mut &'a [u8], n: usize) -> Option<&'a [T]> {
    let len = n * mem::size_of::<T>();
    if cursor.len() < len {
        return None;
    }
    let (before, after) = cursor.split_at(len);
    *cursor = after;
    Some(bytemuck::cast_slice(before))
}

#[cfg(any(windows, feature = "default-text-engine"))]
pub fn pod<P: AnyBitPattern>(bytes: &[u8]) -> Option<&P> {
    Some(bytemuck::from_bytes(bytes.get(..mem::size_of::<P>())?))
}

#[cfg(any(windows, feature = "default-text-engine"))]
pub fn slice<P: AnyBitPattern>(bytes: &[u8], n: usize) -> Option<&[P]> {
    Some(bytemuck::cast_slice(bytes.get(..n * mem::size_of::<P>())?))
}
