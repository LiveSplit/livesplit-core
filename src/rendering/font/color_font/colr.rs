use super::parse_util::{pod, slice, O32, U16};
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Header {
    version: U16,
    num_base_glyph_records: U16,
    base_glyph_records_offset: O32,
    layer_records_offset: O32,
    num_layer_records: U16,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct BaseGlyphRecord {
    glyph_id: U16,
    first_layer_idx: U16,
    num_layers: U16,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct LayerRecord {
    glyph_id: U16,
    palette_entry_idx: U16,
}

impl LayerRecord {
    pub fn glyph_id(&self) -> u16 {
        self.glyph_id.get()
    }

    pub fn palette_entry_idx(&self) -> u16 {
        self.palette_entry_idx.get()
    }
}

pub fn look_up(table: &[u8], glyph: u16) -> Option<&[LayerRecord]> {
    let header = pod::<Header>(table)?;

    let base_glyphs = slice::<BaseGlyphRecord>(
        table.get(header.base_glyph_records_offset.usize()..)?,
        header.num_base_glyph_records.usize(),
    )?;

    let glyph = glyph.to_be_bytes();
    let base_glyph = &base_glyphs[base_glyphs
        .binary_search_by_key(&glyph, |b| b.glyph_id.0)
        .ok()?];

    let layer_records = slice(
        table.get(header.layer_records_offset.usize()..)?,
        header.num_layer_records.usize(),
    )?;

    layer_records
        .get(base_glyph.first_layer_idx.usize()..)?
        .get(..base_glyph.num_layers.usize())
}
