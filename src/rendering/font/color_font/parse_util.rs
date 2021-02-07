use bytemuck::{Pod, Zeroable};
use std::{fmt, mem};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(transparent)]
pub struct U16(pub [u8; 2]);

impl fmt::Debug for U16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.get(), f)
    }
}

impl U16 {
    pub fn get(self) -> u16 {
        u16::from_be_bytes(self.0)
    }

    pub fn usize(self) -> usize {
        self.get() as usize
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(transparent)]
pub struct O32(pub [u8; 4]);

impl fmt::Debug for O32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.get(), f)
    }
}

impl O32 {
    pub fn get(self) -> u32 {
        u32::from_be_bytes(self.0)
    }

    pub fn usize(self) -> usize {
        self.get() as usize
    }
}

pub fn pod<P: Pod>(bytes: &[u8]) -> Option<&P> {
    Some(bytemuck::from_bytes(bytes.get(..mem::size_of::<P>())?))
}

pub fn slice<P: Pod>(bytes: &[u8], n: usize) -> Option<&[P]> {
    Some(bytemuck::cast_slice(bytes.get(..n * mem::size_of::<P>())?))
}
