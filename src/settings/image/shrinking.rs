use alloc::borrow::Cow;
use bytemuck_derive::{Pod, Zeroable};
use image::{
    codecs::{bmp, farbfeld, hdr, ico, jpeg, pnm, tga, tiff, webp},
    guess_format, load_from_memory_with_format, ImageDecoder, ImageEncoder, ImageFormat,
};
use std::io::Cursor;

use crate::util::byte_parsing::{big_endian::U32, strip_pod};

fn shrink_inner(data: &[u8], max_dim: u32) -> Option<Cow<'_, [u8]>> {
    let format = guess_format(data).ok()?;

    let (width, height) = match format {
        ImageFormat::Png => {
            // We encounter a lot of PNG images in splits files and decoding
            // them with image's PNG decoder seems to decode way more than
            // necessary. We really just need to find the width and height. The
            // specification is here:
            // https://www.w3.org/TR/2003/REC-PNG-20031110/
            //
            // And it says the following:
            //
            // "A valid PNG datastream shall begin with a PNG signature,
            // immediately followed by an IHDR chunk".
            //
            // Each chunk is encoded as a length and type and then its chunk
            // encoding. An IHDR chunk immediately starts with the width and
            // height. This means we can model the beginning of a PNG file as
            // follows:
            #[derive(Copy, Clone, Pod, Zeroable)]
            #[repr(C)]
            struct BeginningOfPng {
                // 5.2 PNG signature
                png_signature: [u8; 8],
                // 5.3 Chunk layout
                chunk_len: [u8; 4],
                // 11.2.2 IHDR Image header
                chunk_type: [u8; 4],
                width: U32,
                height: U32,
            }

            // This improves parsing speed of entire splits files by up to 30%.

            let beginning_of_png: &BeginningOfPng = strip_pod(&mut &*data)?;
            (beginning_of_png.width.get(), beginning_of_png.height.get())
        }
        ImageFormat::Jpeg => jpeg::JpegDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::WebP => webp::WebPDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::Pnm => pnm::PnmDecoder::new(data).ok()?.dimensions(),
        ImageFormat::Tiff => tiff::TiffDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::Tga => tga::TgaDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::Bmp => bmp::BmpDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::Ico => ico::IcoDecoder::new(Cursor::new(data)).ok()?.dimensions(),
        ImageFormat::Hdr => hdr::HdrDecoder::new(data).ok()?.dimensions(),
        ImageFormat::Farbfeld => farbfeld::FarbfeldDecoder::new(data).ok()?.dimensions(),
        // FIXME: For GIF we would need to properly shrink the whole animation.
        // The image crate can't properly handle this at this point in time.
        // Some properties are not translated over properly it seems. We could
        // shrink GIFs that are a single frame, but we also can't tell how many
        // frames there are.
        // DDS isn't a format we really care for.
        // AVIF uses C bindings, so it's not portable.
        // The OpenEXR code in the image crate doesn't compile on Big Endian targets.
        // And the image format is non-exhaustive.
        _ => return Some(data.into()),
    };

    let is_too_large = width > max_dim || height > max_dim;
    Some(if is_too_large || format == ImageFormat::Bmp {
        let mut image = load_from_memory_with_format(data, format).ok()?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }
        let mut data = Vec::new();
        image::codecs::png::PngEncoder::new(&mut data)
            .write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color().into(),
            )
            .ok()?;
        data.into()
    } else {
        data.into()
    })
}

pub fn shrink(data: &[u8], max_dim: u32) -> Cow<'_, [u8]> {
    shrink_inner(data, max_dim).unwrap_or_else(|| data.into())
}
