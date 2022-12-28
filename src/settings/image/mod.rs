use crate::platform::prelude::*;
use core::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(test)]
mod tests;

#[cfg(all(feature = "std", feature = "image-shrinking"))]
mod shrinking;

static LAST_IMAGE_ID: AtomicUsize = AtomicUsize::new(0);

/// Images can be used to store segment and game icons. Each image object comes
/// with an ID that changes whenever the image is modified. IDs are unique
/// across different images. There's no specific image format you need to use
/// for the images.
#[derive(Debug, Clone)]
pub struct Image {
    data: Vec<u8>,
    id: usize,
}

/// Describes an owned representation of an image's data. It is suitable to be
/// used in state objects. It can efficiently be serialized for various formats.
/// For binary formats it gets serialized as its raw byte representation, while
/// for textual formats it gets serialized as a Base64 Data URL instead.
#[derive(Debug, Clone)]
pub struct ImageData(pub Box<[u8]>);

impl<T> From<T> for ImageData
where
    Box<[u8]>: From<T>,
{
    fn from(value: T) -> Self {
        ImageData(value.into())
    }
}

impl Deref for ImageData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for ImageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            if !self.0.is_empty() {
                let mut buf = String::from("data:;base64,");

                // SAFETY: We encode Base64 to the end of the string, which is
                // always valid UTF-8. Once we've written it, we simply increase
                // the length of the buffer by the amount of bytes written.
                unsafe {
                    let buf = buf.as_mut_vec();
                    let encoded_len = base64_simd::STANDARD.encoded_length(self.0.len());
                    buf.reserve_exact(encoded_len);
                    let additional_len = base64_simd::STANDARD
                        .encode(
                            &self.0,
                            base64_simd::Out::from_uninit_slice(buf.spare_capacity_mut()),
                        )
                        .len();
                    buf.set_len(buf.len() + additional_len);
                }

                serializer.serialize_str(&buf)
            } else {
                serializer.serialize_str("")
            }
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for ImageData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let data: &'de str = Deserialize::deserialize(deserializer)?;
            if data.is_empty() {
                Ok(ImageData(Box::new([])))
            } else if let Some(encoded_image_data) = data.strip_prefix("data:;base64,") {
                let image_data = base64_simd::STANDARD
                    .decode_to_vec(encoded_image_data.as_bytes())
                    .map_err(de::Error::custom)?;

                Ok(ImageData(image_data.into_boxed_slice()))
            } else {
                Err(de::Error::custom("Invalid Data URL for image"))
            }
        } else {
            Ok(ImageData(Deserialize::deserialize(deserializer)?))
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.id == other.id || self.data == other.data
    }
}

impl Default for Image {
    fn default() -> Image {
        Image::new(&[])
    }
}

impl<D: AsRef<[u8]>> From<D> for Image {
    fn from(d: D) -> Self {
        Image::new(d.as_ref())
    }
}

impl Image {
    /// Creates a new image with a unique ID with the image data provided.
    pub fn new(data: &[u8]) -> Self {
        let mut image = Image {
            data: Vec::new(),
            id: 0,
        };
        image.modify(data);
        image
    }

    /// Loads an image from the file system. You need to provide a buffer used
    /// for temporarily storing the image's data.
    #[cfg(feature = "std")]
    pub fn from_file<P>(path: P, buf: &mut Vec<u8>) -> std::io::Result<Image>
    where
        P: AsRef<std::path::Path>,
    {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        buf.clear();
        file.read_to_end(buf)?;

        Ok(Image::new(buf))
    }

    /// Accesses the unique ID for this image.
    #[inline]
    pub const fn id(&self) -> usize {
        self.id
    }

    /// Accesses the image's data. If the image's data is empty, this returns an
    /// empty slice.
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Modifies an image by replacing its image data with the new image data
    /// provided. The image's ID changes to a new unique ID.
    pub fn modify(&mut self, data: &[u8]) {
        #[cfg(all(feature = "std", feature = "image-shrinking"))]
        let data = {
            const MAX_IMAGE_SIZE: u32 = 128;

            shrinking::shrink(data, MAX_IMAGE_SIZE)
        };
        cfg_if::cfg_if! {
            if #[cfg(target_has_atomic = "ptr")] {
                self.id = LAST_IMAGE_ID.fetch_add(1, Ordering::Relaxed);
            } else {
                self.id = LAST_IMAGE_ID.load(Ordering::SeqCst) + 1;
                LAST_IMAGE_ID.store(self.id, Ordering::SeqCst);
            }
        }
        self.data.clear();
        self.data.extend_from_slice(&data);
    }

    /// Checks if the image data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// With a Cached Image ID you can track image changes. It starts with an
/// uncached state and then gets updated with the images provided to it. It can
/// be reset at any point in order to force a change to be detected.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum CachedImageId {
    /// The initial uncached state.
    #[default]
    Uncached,
    /// The last image observed either was missing or contained no data.
    NoImage,
    /// The last image had actual data and the ID stored here.
    Image(usize),
}

impl CachedImageId {
    /// Updates the cached image ID based on the optional image provided to this
    /// method. If a change is observed the image's data is returned. An empty
    /// slice is returned when a transition to no image or no image data is
    /// observed.
    pub fn update_with<'i>(&mut self, image: Option<&'i Image>) -> Option<&'i [u8]> {
        let new_value = image.map_or(CachedImageId::NoImage, |i| {
            if i.is_empty() {
                CachedImageId::NoImage
            } else {
                CachedImageId::Image(i.id())
            }
        });

        if *self != new_value {
            *self = new_value;
            Some(image.map_or(&[], |i| &i.data))
        } else {
            None
        }
    }

    /// Resets the state of the cached image ID to uncached.
    pub fn reset(&mut self) {
        *self = CachedImageId::Uncached;
    }
}
