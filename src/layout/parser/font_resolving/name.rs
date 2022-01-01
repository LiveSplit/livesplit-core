use std::mem;

use super::parse_util::{pod, slice, U16};
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Header {
    version: U16,
    count: U16,
    storage_offset: U16,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct NameRecord {
    platform_id: U16,
    encoding_id: U16,
    language_id: U16,
    name_id: U16,
    length: U16,
    string_offset: U16,
}

impl NameRecord {
    fn get_name(&self, storage: &[u8]) -> Option<String> {
        let name = storage
            .get(self.string_offset.usize()..)?
            .get(..self.length.usize())?;

        let mut buf = Vec::new();
        let slice: &[[u8; 2]] = bytemuck::try_cast_slice(name).ok()?;
        for &c in slice {
            buf.push(u16::from_be_bytes(c));
        }

        String::from_utf16(&buf).ok()
    }
}

const fn is_unicode_encoding(platform_id: u16, encoding_id: u16) -> bool {
    match platform_id {
        0 => true,
        3 => matches!(encoding_id, 0 | 1),
        _ => false,
    }
}

pub fn look_up_family_name(table: &[u8]) -> Option<String> {
    let header = pod::<Header>(table)?;
    let records =
        slice::<NameRecord>(table.get(mem::size_of::<Header>()..)?, header.count.usize())?;

    let font_family = 1u16.to_be_bytes();
    let typographic_family = 16u16.to_be_bytes();

    let record = records
        .iter()
        .filter(|r| r.name_id.0 == font_family || r.name_id.0 == typographic_family)
        .filter(|r| is_unicode_encoding(r.platform_id.get(), r.encoding_id.get()))
        .filter(|r| match r.platform_id.get() {
            0 => true,
            1 => r.language_id.get() == 0,
            3 => r.language_id.get() & 0xFF == 0x09,
            _ => false,
        })
        .max_by_key(|r| (r.name_id.0, !r.platform_id.get()))?;

    let storage = table.get(header.storage_offset.usize()..)?;
    record.get_name(storage)
}
