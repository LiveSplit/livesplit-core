use alloc::borrow::Cow;
use image::{
    bmp, gif, guess_format, hdr, ico, imageops, jpeg, load_from_memory_with_format, png, pnm, tiff,
    webp, AnimationDecoder, DynamicImage, Frame, GenericImageView, ImageDecoder, ImageError,
    ImageFormat,
};
use std::io::Cursor;

fn shrink_inner(data: &[u8], max_dim: u32) -> Result<Cow<'_, [u8]>, ImageError> {
    let format = guess_format(data)?;

    let cursor = Cursor::new(data);
    let (width, height) = match format {
        ImageFormat::Png => png::PngDecoder::new(cursor)?.dimensions(),
        ImageFormat::Jpeg => jpeg::JpegDecoder::new(cursor)?.dimensions(),
        ImageFormat::Gif => {
            let decoder = gif::GifDecoder::new(cursor)?;
            let (width, height) = decoder.dimensions();
            return if width > max_dim || height > max_dim {
                let mut data = Vec::new();
                gif::Encoder::new(&mut data).try_encode_frames(decoder.into_frames().map(
                    |image| {
                        let image = image?;
                        let buffer = image.buffer();
                        let scaled_down = imageops::thumbnail(buffer, max_dim, max_dim);

                        let scale_factor = if buffer.width() > buffer.height() {
                            buffer.width() as f64 / scaled_down.width() as f64
                        } else {
                            buffer.height() as f64 / scaled_down.height() as f64
                        };

                        let left = scale_factor * image.left() as f64;
                        let top = scale_factor * image.top() as f64;

                        Ok(Frame::from_parts(
                            scaled_down,
                            left as _,
                            top as _,
                            image.delay(),
                        ))
                    },
                ))?;
                Ok(data.into())
            } else {
                Ok(data.into())
            };
        }
        ImageFormat::WebP => webp::WebPDecoder::new(cursor)?.dimensions(),
        ImageFormat::Tiff => tiff::TiffDecoder::new(cursor)?.dimensions(),
        ImageFormat::Bmp => bmp::BmpDecoder::new(cursor)?.dimensions(),
        ImageFormat::Ico => ico::IcoDecoder::new(cursor)?.dimensions(),
        ImageFormat::Hdr => hdr::HDRAdapter::new(cursor)?.dimensions(),
        ImageFormat::Pnm => pnm::PnmDecoder::new(cursor)?.dimensions(),
        // TGA doesn't have a Header.
        // DDS isn't a format we really care for.
        // And the image format is non-exhaustive.
        ImageFormat::Tga | ImageFormat::Dds | _ => return Ok(data.into()),
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
        png::PNGEncoder::new(&mut data).encode(
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
