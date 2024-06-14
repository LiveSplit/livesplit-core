use alloc::borrow::Cow;
use image::{guess_format, load_from_memory_with_format, ImageEncoder, ImageFormat};

use crate::util::image::{create_reencoder, get_dimensions};

fn shrink_inner(data: &[u8], max_dim: u32) -> Option<Cow<'_, [u8]>> {
    let format = guess_format(data).ok()?;

    let dims = if format == ImageFormat::Gif {
        // FIXME: For GIF we would need to properly shrink the whole animation.
        // The image crate can't properly handle this at this point in time.
        // Some properties are not translated over properly it seems. We could
        // shrink GIFs that are a single frame, but we also can't tell how many
        // frames there are.
        None
    } else {
        get_dimensions(format, data)
    };

    let Some((width, height)) = dims else {
        return Some(data.into());
    };

    let is_too_large = width > max_dim || height > max_dim;
    Some(if is_too_large || format == ImageFormat::Bmp {
        let mut image = load_from_memory_with_format(data, format).ok()?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }

        let mut data = Vec::new();

        create_reencoder(&mut data)
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
