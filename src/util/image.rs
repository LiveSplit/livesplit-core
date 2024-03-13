use std::io::Cursor;

use bytemuck_derive::{Pod, Zeroable};
use image::{
    codecs::{bmp, farbfeld, hdr, ico, jpeg, pnm, tga, tiff, webp},
    ImageDecoder, ImageFormat,
};

use crate::util::byte_parsing::{big_endian::U32, strip_pod};

pub fn get_dimensions(format: ImageFormat, data: &[u8]) -> Option<(u32, u32)> {
    Some(match format {
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
        #[cfg(feature = "rendering")]
        ImageFormat::Gif => image::codecs::gif::GifDecoder::new(Cursor::new(data))
            .ok()?
            .dimensions(),
        // DDS isn't a format we really care for.
        // AVIF uses C bindings, so it's not portable.
        // The OpenEXR code in the image crate doesn't compile on Big Endian targets.
        // And the image format is non-exhaustive.
        _ => return None,
    })
}
