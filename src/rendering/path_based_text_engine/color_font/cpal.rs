use std::mem;

use super::parse_util::{pod, slice, O32, U16};
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Header {
    version: U16,
    num_palette_entries: U16,
    num_palettes: U16,
    num_color_records: U16,
    color_records_array_offset: O32,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct HeaderV1Extra {
    palette_types_array_offset: O32,
    palette_labels_array_offset: O32,
    palette_entry_labels_array_offset: O32,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ColorRecord {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

pub fn look_up(table: &[u8], palette_idx: usize, palette_entry_idx: u16) -> Option<&ColorRecord> {
    let header = pod::<Header>(table)?;
    let color_record_indices = slice::<U16>(
        table.get(mem::size_of::<Header>()..)?,
        header.num_palettes.usize(),
    )?;
    let color_records = slice::<ColorRecord>(
        table.get(header.color_records_array_offset.usize()..)?,
        header.num_color_records.usize(),
    )?;
    let color_record_index = color_record_indices.get(palette_idx)?.usize();
    color_records.get(color_record_index + palette_entry_idx as usize)
}
