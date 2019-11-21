use alloc::borrow::Cow;
use image::{
    bmp, gif, guess_format, hdr, ico, jpeg, load_from_memory_with_format, png, pnm, tiff, webp,
    DynamicImage, ImageDecoder, ImageError, ImageFormat,
};
use std::io::Cursor;

fn shrink_inner(data: &[u8], max_dim: u32) -> Result<Cow<'_, [u8]>, ImageError> {
    let format = guess_format(data)?;

    let cursor = Cursor::new(data);
    let (width, height) = match format {
        ImageFormat::PNG => png::PNGDecoder::new(cursor)?.dimensions(),
        ImageFormat::JPEG => jpeg::JPEGDecoder::new(cursor)?.dimensions(),
        ImageFormat::GIF => gif::Decoder::new(cursor)?.dimensions(),
        ImageFormat::WEBP => webp::WebpDecoder::new(cursor)?.dimensions(),
        ImageFormat::TIFF => tiff::TIFFDecoder::new(cursor)?.dimensions(),
        ImageFormat::BMP => bmp::BMPDecoder::new(cursor)?.dimensions(),
        ImageFormat::ICO => ico::ICODecoder::new(cursor)?.dimensions(),
        ImageFormat::HDR => hdr::HDRAdapter::new(cursor)?.dimensions(),
        ImageFormat::PNM => pnm::PNMDecoder::new(cursor)?.dimensions(),
        ImageFormat::TGA => return Ok(data.into()), // TGA doesn't have a Header
    };

    let is_too_large = width > u64::from(max_dim) || height > u64::from(max_dim);
    if is_too_large || format == ImageFormat::BMP {
        let mut image = load_from_memory_with_format(data, format)?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }
        let mut data = Vec::new();
        let ((width, height), image_data) = match &image {
            DynamicImage::ImageLuma8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageLumaA8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageRgb8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageRgba8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageBgr8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageBgra8(x) => (x.dimensions(), x.as_ref()),
        };
        png::PNGEncoder::new(&mut data).encode(image_data, width, height, image.color())?;
        Ok(data.into())
    } else {
        Ok(data.into())
    }
}

pub fn shrink(data: &[u8], max_dim: u32) -> Cow<'_, [u8]> {
    shrink_inner(data, max_dim).unwrap_or_else(|_| data.into())
}
