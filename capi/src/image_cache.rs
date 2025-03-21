//! A cache for images that allows looking up images by their ID. The cache uses
//! a garbage collection algorithm to remove images that have not been visited
//! since the last garbage collection. Functions updating the cache usually
//! don't run the garbage collection themselves, so make sure to call `collect`
//! every now and then to remove unvisited images.

use std::{ffi::c_char, ptr, str::FromStr};

use livesplit_core::settings::{HasImageId, Image, ImageCache, ImageId};

use crate::{output_str, slice, str};

/// type
pub type OwnedImageCache = Box<ImageCache>;
/// type
pub type NullableOwnedImageCache = Option<OwnedImageCache>;

/// Creates a new image cache.
#[unsafe(no_mangle)]
pub extern "C" fn ImageCache_new() -> OwnedImageCache {
    Box::new(ImageCache::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn ImageCache_drop(this: OwnedImageCache) {
    drop(this);
}

/// Looks up an image in the cache based on its image ID and returns a pointer
/// to the bytes that make up the image. The bytes are the image in its original
/// file format. The format is not specified and can be any image format. The
/// data may not even represent a valid image at all. If the image is not in the
/// cache, <NULL> is returned. This does not mark the image as visited.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ImageCache_lookup_data_ptr(
    this: &ImageCache,
    key: *const c_char,
) -> *const u8 {
    // SAFETY: The caller guarantees that `key` is valid.
    ImageId::from_str(unsafe { str(key) })
        .ok()
        .and_then(|key| this.lookup(&key))
        .filter(|image| !image.is_empty())
        .map(|image| image.data().as_ptr())
        .unwrap_or(ptr::null())
}

/// Looks up an image in the cache based on its image ID and returns its byte
/// length. If the image is not in the cache, 0 is returned. This does not mark
/// the image as visited.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ImageCache_lookup_data_len(
    this: &ImageCache,
    key: *const c_char,
) -> usize {
    // SAFETY: The caller guarantees that `key` is valid.
    ImageId::from_str(unsafe { str(key) })
        .ok()
        .and_then(|key| this.lookup(&key))
        .map(|image| image.data().len())
        .unwrap_or_default()
}

/// Caches an image and returns its image ID. The image is provided as a byte
/// array. The image ID is the hash of the image data and can be used to look up
/// the image in the cache. The image is marked as visited in the cache. If you
/// specify that the image is large, it gets considered a large image that may
/// be used as a background image. Otherwise it gets considered an icon. The
/// image is resized according to this information.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ImageCache_cache(
    this: &mut ImageCache,
    data: *const u8,
    len: usize,
    is_large: bool,
) -> *const c_char {
    // SAFETY: The caller guarantees that `data` is valid.
    let image = Image::new(
        unsafe { slice(data, len).into() },
        if is_large { Image::LARGE } else { Image::ICON },
    );
    let image_id = *image.image_id();
    this.cache(&image_id, || image);
    let mut buf = [0; 64];
    output_str(image_id.format_str(&mut buf))
}

/// Runs the garbage collection of the cache. This removes images from the cache
/// that have not been visited since the last garbage collection. Not every
/// image that has not been visited is removed. There is a heuristic that keeps
/// a certain amount of images in the cache regardless of whether they have been
/// visited or not. Returns the amount of images that got collected.
#[unsafe(no_mangle)]
pub extern "C" fn ImageCache_collect(this: &mut ImageCache) -> usize {
    this.collect()
}
