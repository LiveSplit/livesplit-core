use core::mem;

use bytemuck::AnyBitPattern;

pub fn strip_u8(cursor: &mut &[u8]) -> Option<u8> {
    strip_pod(cursor).copied()
}

pub fn strip_be_u16(cursor: &mut &[u8]) -> Option<u16> {
    Some(u16::from_be_bytes(*strip_pod(cursor)?))
}

pub fn strip_be_u32(cursor: &mut &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(*strip_pod(cursor)?))
}

pub fn strip_be_u64(cursor: &mut &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(*strip_pod(cursor)?))
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
