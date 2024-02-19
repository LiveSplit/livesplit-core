use crate::platform::{prelude::*, Arc};
use core::{fmt, ops::Deref};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use sha2::{Digest, Sha256};

#[cfg(test)]
mod tests;

mod cache;
mod image_id;
#[cfg(all(feature = "std", feature = "image-shrinking"))]
mod shrinking;

pub use cache::{HasImageId, ImageCache};
pub use image_id::ImageId;

/// Images can be used to store segment and game icons. Each image object comes
/// with a strong hash to quickly compare images. There's no specific image
/// format you need to use for the images.
#[derive(Clone)]
pub struct Image {
    data: Option<Arc<[u8]>>,
    id: ImageId,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("is_empty", &self.is_empty())
            .field("id", &self.id)
            .finish()
    }
}

impl Deref for Image {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.data.as_deref().unwrap_or_default()
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = self.data();
        if serializer.is_human_readable() {
            if !data.is_empty() {
                let base64 = base64_simd::STANDARD.encode_to_string(data);
                serializer.serialize_str(&base64)
            } else {
                serializer.serialize_str("")
            }
        } else {
            serializer.serialize_bytes(data)
        }
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(ImageVisitor)
        } else {
            deserializer.deserialize_bytes(ImageVisitor)
        }
    }
}

struct ImageVisitor;

impl<'de> Visitor<'de> for ImageVisitor {
    type Value = Image;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a base64 encoded image or its bytes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Image, E>
    where
        E: de::Error,
    {
        if value.is_empty() {
            Ok(Image::new_inner([].into()))
        } else {
            let image_data = base64_simd::STANDARD
                .decode_to_vec(value.as_bytes())
                .map_err(de::Error::custom)?;

            Ok(Image::new_inner(image_data.into()))
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Image::new_inner(v.into()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Image::new_inner(v.into()))
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::EMPTY.clone()
    }
}

impl Image {
    /// The maximum size of an image that's used as an icon.
    pub const ICON: u32 = 128;
    /// The maximum size of an image that is meant to be large.
    pub const LARGE: u32 = 512;

    /// An empty image.
    pub const EMPTY: &'static Self = &Self {
        data: None,
        id: *ImageId::EMPTY,
    };

    fn new_inner(data: Arc<[u8]>) -> Self {
        let hash = Sha256::digest(&*data);
        Self {
            data: Some(data),
            id: ImageId(hash.as_slice().try_into().unwrap()),
        }
    }

    /// Creates a new image with the image data provided.
    pub fn new(data: Arc<[u8]>, max_image_size: u32) -> Self {
        let _ = max_image_size;
        #[cfg(all(feature = "std", feature = "image-shrinking"))]
        let data = {
            match shrinking::shrink(&data, max_image_size) {
                alloc::borrow::Cow::Borrowed(_) => data,
                alloc::borrow::Cow::Owned(data) => data.into(),
            }
        };
        Self::new_inner(data)
    }

    /// Loads an image from the file system. You need to provide a buffer used
    /// for temporarily storing the image's data.
    #[cfg(feature = "std")]
    pub fn from_file<P>(path: P, buf: &mut Vec<u8>, max_image_size: u32) -> std::io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        buf.clear();
        file.read_to_end(buf)?;

        Ok(Self::new(buf.as_slice().into(), max_image_size))
    }

    /// Accesses the image's data. If the image's data is empty, this returns an
    /// empty slice.
    #[inline]
    pub fn data(&self) -> &[u8] {
        self
    }

    /// Accesses the image's ID. This is a unique identifier for the image. It
    /// is implemented via a SHA-256 hash.
    #[inline]
    pub const fn id(&self) -> &ImageId {
        &self.id
    }

    /// Checks if the image data is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data().is_empty()
    }
}
