use alloc::borrow::Cow;
use image::{
    codecs::{bmp, hdr, ico, jpeg, png, pnm, tiff, webp},
    guess_format, load_from_memory_with_format, DynamicImage, GenericImageView, ImageDecoder,
    ImageError, ImageFormat,
};
use std::io::Cursor;

fn shrink_inner(data: &[u8], max_dim: u32) -> Result<Cow<'_, [u8]>, ImageError> {
    let format = guess_format(data)?;

    let cursor = Cursor::new(data);
    let (width, height) = match format {
        ImageFormat::Png => png::PngDecoder::new(cursor)?.dimensions(),
        ImageFormat::Jpeg => jpeg::JpegDecoder::new(cursor)?.dimensions(),
        ImageFormat::WebP => webp::WebPDecoder::new(cursor)?.dimensions(),
        ImageFormat::Tiff => tiff::TiffDecoder::new(cursor)?.dimensions(),
        ImageFormat::Bmp => bmp::BmpDecoder::new(cursor)?.dimensions(),
        ImageFormat::Ico => ico::IcoDecoder::new(cursor)?.dimensions(),
        ImageFormat::Hdr => hdr::HdrAdapter::new(cursor)?.dimensions(),
        ImageFormat::Pnm => pnm::PnmDecoder::new(cursor)?.dimensions(),
        // FIXME: For GIF we would need to properly shrink the whole animation.
        // The image crate can't properly handle this at this point in time.
        // Some properties are not translated over properly it seems. We could
        // shrink GIFs that are a single frame, but we also can't tell how many
        // frames there are.
        // TGA doesn't have a Header.
        // DDS isn't a format we really care for.
        // And the image format is non-exhaustive.
        _ => return Ok(data.into()),
    };

    let is_too_large = width > max_dim || height > max_dim;
    if is_too_large || format == ImageFormat::Bmp {
        let mut image = load_from_memory_with_format(data, format)?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }
        let image_data = match &image {
            DynamicImage::ImageLuma8(x) => x.as_ref(),
            DynamicImage::ImageLumaA8(x) => x.as_ref(),
            DynamicImage::ImageRgb8(x) => x.as_ref(),
            DynamicImage::ImageRgba8(x) => x.as_ref(),
            DynamicImage::ImageBgr8(x) => x.as_ref(),
            DynamicImage::ImageBgra8(x) => x.as_ref(),
            DynamicImage::ImageLuma16(x) => bytemuck::cast_slice(x.as_ref()),
            DynamicImage::ImageLumaA16(x) => bytemuck::cast_slice(x.as_ref()),
            DynamicImage::ImageRgb16(x) => bytemuck::cast_slice(x.as_ref()),
            DynamicImage::ImageRgba16(x) => bytemuck::cast_slice(x.as_ref()),
        };
        let mut data = Vec::new();
        png::PngEncoder::new(&mut data).encode(
            image_data,
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
