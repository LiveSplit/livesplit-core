//! Linux-specific module size resolution.
//!
//! `/proc/<pid>/maps` describes virtual memory areas rather than logical
//! executable images. This distinction matters for Windows executables running
//! through Wine or Proton: Wine maps a PE image as multiple regions with
//! different protections and backing. Only some of those regions retain the
//! executable or DLL path, while other PE sections are anonymous mappings.
//! Summing only the regions with a matching path therefore often reports just
//! one or two pages instead of the image's full virtual extent.
//!
//! The path is still the most useful way to discover the module and its base
//! address, so the regular process module lookup remains unchanged. Once the
//! base is known, a valid in-memory PE image gets its logical size from the
//! optional header's `SizeOfImage` field. This is the extent that module-relative
//! addresses and signature scans expect, including any unreadable or uncommitted
//! holes between sections.
//!
//! Native Linux modules do not have PE headers, and PE headers may be malformed
//! or temporarily unreadable. In those cases, module sizing falls back to the
//! sum of the path-matched mappings, preserving the previous behavior.

use std::io;

use super::{Address, Process};

const DOS_HEADER_SIZE: usize = 0x40;
const PE_AND_COFF_HEADER_SIZE: usize = 0x18;
const OPTIONAL_HEADER_PREFIX_SIZE: usize = 0x40;
const PE32_MAGIC: u16 = 0x10B;
const PE32_PLUS_MAGIC: u16 = 0x20B;

pub(super) fn module_size(process: &Process, module_address: Address, mapped_size: u64) -> u64 {
    logical_module_size(module_address, mapped_size, |address, buf| {
        process.read_mem(address, buf)
    })
}

fn logical_module_size(
    module_address: Address,
    mapped_size: u64,
    read_memory: impl FnMut(Address, &mut [u8]) -> io::Result<()>,
) -> u64 {
    read_pe_size_of_image(module_address, read_memory).unwrap_or(mapped_size)
}

fn read_pe_size_of_image(
    module_address: Address,
    mut read_memory: impl FnMut(Address, &mut [u8]) -> io::Result<()>,
) -> Option<u64> {
    let mut dos_header = [0; DOS_HEADER_SIZE];
    read_memory(module_address, &mut dos_header).ok()?;
    if &dos_header[..2] != b"MZ" {
        return None;
    }

    let pe_offset = read_u32(&dos_header, 0x3C)?;
    if pe_offset < DOS_HEADER_SIZE as u32 {
        return None;
    }
    let pe_address = module_address.checked_add(pe_offset.into())?;

    let mut pe_and_coff_header = [0; PE_AND_COFF_HEADER_SIZE];
    read_memory(pe_address, &mut pe_and_coff_header).ok()?;
    if &pe_and_coff_header[..4] != b"PE\0\0" {
        return None;
    }

    let number_of_sections = read_u16(&pe_and_coff_header, 6)?;
    if !(1..=96).contains(&number_of_sections) {
        return None;
    }

    let optional_header_size = read_u16(&pe_and_coff_header, 20)?;
    let optional_header_address = pe_address.checked_add(PE_AND_COFF_HEADER_SIZE as u64)?;
    let mut optional_header = [0; OPTIONAL_HEADER_PREFIX_SIZE];
    read_memory(optional_header_address, &mut optional_header).ok()?;

    let optional_header_magic = read_u16(&optional_header, 0)?;
    let minimum_optional_header_size = match optional_header_magic {
        PE32_MAGIC => 0x60,
        PE32_PLUS_MAGIC => 0x70,
        _ => return None,
    };
    if optional_header_size < minimum_optional_header_size {
        return None;
    }

    let section_alignment = read_u32(&optional_header, 0x20)?;
    let size_of_image = read_u32(&optional_header, 0x38)?;
    let size_of_headers = read_u32(&optional_header, 0x3C)?;
    let header_end = pe_offset
        .checked_add(PE_AND_COFF_HEADER_SIZE as u32)?
        .checked_add(optional_header_size.into())?;

    if section_alignment == 0
        || size_of_image == 0
        || !size_of_image.is_multiple_of(section_alignment)
        || size_of_headers == 0
        || size_of_headers > size_of_image
        || header_end > size_of_headers
    {
        return None;
    }

    Some(size_of_image.into())
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        bytes.get(offset..offset.checked_add(2)?)?.try_into().ok()?,
    ))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_ADDRESS: Address = 0x0001_4000_0000;
    const MAPPED_SIZE: u64 = 0x1000;
    const SIZE_OF_IMAGE: u32 = 0x1_2000;

    fn pe_image(magic: u16) -> Vec<u8> {
        let pe_offset = 0x80usize;
        let optional_header_size = match magic {
            PE32_MAGIC => 0xE0u16,
            PE32_PLUS_MAGIC => 0xF0u16,
            _ => 0xF0u16,
        };
        let optional_header_offset = pe_offset + PE_AND_COFF_HEADER_SIZE;
        let mut image = vec![0; optional_header_offset + OPTIONAL_HEADER_PREFIX_SIZE];

        image[..2].copy_from_slice(b"MZ");
        image[0x3C..0x40].copy_from_slice(&(pe_offset as u32).to_le_bytes());
        image[pe_offset..pe_offset + 4].copy_from_slice(b"PE\0\0");
        image[pe_offset + 6..pe_offset + 8].copy_from_slice(&1u16.to_le_bytes());
        image[pe_offset + 20..pe_offset + 22].copy_from_slice(&optional_header_size.to_le_bytes());
        image[optional_header_offset..optional_header_offset + 2]
            .copy_from_slice(&magic.to_le_bytes());
        image[optional_header_offset + 0x20..optional_header_offset + 0x24]
            .copy_from_slice(&0x1000u32.to_le_bytes());
        image[optional_header_offset + 0x38..optional_header_offset + 0x3C]
            .copy_from_slice(&SIZE_OF_IMAGE.to_le_bytes());
        image[optional_header_offset + 0x3C..optional_header_offset + 0x40]
            .copy_from_slice(&0x400u32.to_le_bytes());

        image
    }

    fn module_size(image: &[u8]) -> u64 {
        logical_module_size(BASE_ADDRESS, MAPPED_SIZE, |address, buf| {
            let offset = address
                .checked_sub(BASE_ADDRESS)
                .and_then(|offset| usize::try_from(offset).ok())
                .ok_or(io::ErrorKind::InvalidInput)?;
            let end = offset
                .checked_add(buf.len())
                .ok_or(io::ErrorKind::InvalidInput)?;
            let source = image.get(offset..end).ok_or(io::ErrorKind::UnexpectedEof)?;
            buf.copy_from_slice(source);
            Ok(())
        })
    }

    #[test]
    fn reads_pe32_size_of_image() {
        assert_eq!(module_size(&pe_image(PE32_MAGIC)), SIZE_OF_IMAGE.into());
    }

    #[test]
    fn reads_pe32_plus_size_of_image() {
        assert_eq!(
            module_size(&pe_image(PE32_PLUS_MAGIC)),
            SIZE_OF_IMAGE.into()
        );
    }

    #[test]
    fn wine_layout_uses_image_extent_instead_of_named_mapping_size() {
        assert!(u64::from(SIZE_OF_IMAGE) > MAPPED_SIZE);
        assert_eq!(
            module_size(&pe_image(PE32_PLUS_MAGIC)),
            SIZE_OF_IMAGE.into()
        );
    }

    #[test]
    fn malformed_or_truncated_headers_fall_back_to_mapped_size() {
        let valid_image = pe_image(PE32_PLUS_MAGIC);
        for truncated_len in [0, 2, DOS_HEADER_SIZE, 0x80, 0x98, valid_image.len() - 1] {
            assert_eq!(module_size(&valid_image[..truncated_len]), MAPPED_SIZE);
        }

        let mut invalid_mz = valid_image.clone();
        invalid_mz[..2].copy_from_slice(b"ZZ");
        assert_eq!(module_size(&invalid_mz), MAPPED_SIZE);

        let mut invalid_pe = valid_image.clone();
        invalid_pe[0x80..0x84].copy_from_slice(b"PX\0\0");
        assert_eq!(module_size(&invalid_pe), MAPPED_SIZE);

        let mut invalid_optional_header = valid_image.clone();
        invalid_optional_header[0x98..0x9A].copy_from_slice(&0u16.to_le_bytes());
        assert_eq!(module_size(&invalid_optional_header), MAPPED_SIZE);
    }

    #[test]
    fn non_pe_module_falls_back_to_mapped_size() {
        assert_eq!(module_size(b"\x7fELF"), MAPPED_SIZE);
    }
}
