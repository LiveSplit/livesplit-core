use alloc::borrow::Cow;
use image::{
    codecs::{bmp, farbfeld, hdr, ico, jpeg, png, pnm, tga, tiff, webp},
    guess_format, load_from_memory_with_format, ImageDecoder, ImageEncoder, ImageError,
    ImageFormat,
};
use std::io::Cursor;

fn shrink_inner(data: &[u8], max_dim: u32) -> Result<Cow<'_, [u8]>, ImageError> {
    let format = guess_format(data)?;

    let cursor = Cursor::new(data);
    let (width, height) = match format {
        ImageFormat::Png => png::PngDecoder::new(cursor)?.dimensions(),
        ImageFormat::Jpeg => jpeg::JpegDecoder::new(cursor)?.dimensions(),
        ImageFormat::WebP => webp::WebPDecoder::new(cursor)?.dimensions(),
        ImageFormat::Pnm => pnm::PnmDecoder::new(cursor)?.dimensions(),
        ImageFormat::Tiff => tiff::TiffDecoder::new(cursor)?.dimensions(),
        ImageFormat::Tga => tga::TgaDecoder::new(cursor)?.dimensions(),
        ImageFormat::Bmp => bmp::BmpDecoder::new(cursor)?.dimensions(),
        ImageFormat::Ico => ico::IcoDecoder::new(cursor)?.dimensions(),
        ImageFormat::Hdr => hdr::HdrAdapter::new(cursor)?.dimensions(),
        ImageFormat::Farbfeld => farbfeld::FarbfeldDecoder::new(cursor)?.dimensions(),
        // FIXME: For GIF we would need to properly shrink the whole animation.
        // The image crate can't properly handle this at this point in time.
        // Some properties are not translated over properly it seems. We could
        // shrink GIFs that are a single frame, but we also can't tell how many
        // frames there are.
        // DDS isn't a format we really care for.
        // AVIF uses C bindings, so it's not portable.
        // The OpenEXR code in the image crate doesn't compile on Big Endian targets.
        // And the image format is non-exhaustive.
        _ => return Ok(data.into()),
    };

    let is_too_large = width > max_dim || height > max_dim;
    if is_too_large || format == ImageFormat::Bmp {
        let mut image = load_from_memory_with_format(data, format)?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }
        let mut data = Vec::new();
        png::PngEncoder::new(&mut data).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color(),
        )?;
        Ok(data.into())
    } else {
        Ok(data.into())
    }
}

pub fn shrink(data: &[u8], max_dim: u32) -> Cow<'_, [u8]> {
    shrink_inner(data, max_dim).unwrap_or_else(|_| data.into())
}
