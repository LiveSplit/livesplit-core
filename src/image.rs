use base64::{self, STANDARD};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static LAST_IMAGE_ID: AtomicUsize = AtomicUsize::new(0);

/// Images can be used to store segment and game icons. Each image object comes
/// with an ID that changes whenever the image is modified. IDs are unique
/// across different images. You can query the image's data as a Data URL.
/// There's no specific image format you need to use for the images.
#[derive(Debug, Clone)]
pub struct Image {
    url: String,
    id: usize,
}

impl PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.url == other.url
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
            url: String::new(),
            id: 0,
        };
        image.modify(data);
        image
    }

    /// Loads an image from the file system. You need to provide a buffer used
    /// for temporarily storing the image's data.
    pub fn from_file<P, B>(path: P, mut buf: B) -> io::Result<Image>
    where
        P: AsRef<Path>,
        B: AsMut<Vec<u8>>,
    {
        let mut file = File::open(path)?;
        let len = file.metadata()?.len() as usize;

        let buf = buf.as_mut();
        buf.clear();
        buf.reserve(len);
        unsafe {
            buf.set_len(len);
        }

        file.read_exact(buf).map_err(|e| {
            // Avoid exposing the uninitialized bytes and potentially causing
            // Undefined Behavior.
            unsafe { buf.set_len(0) };
            e
        })?;

        Ok(Image::new(buf))
    }

    /// Accesses the unique ID for this image.
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Accesses the Data URL storing the image's data. If the image's data is
    /// empty, this returns an empty string instead of a URL.
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Modifies an image by replacing its image data with the new image data
    /// provided. The image's ID changes to a new unique ID.
    pub fn modify(&mut self, data: &[u8]) {
        #[cfg(feature = "image-shrinking")]
        let data = {
            use crate::image_shrinking::shrink;
            const MAX_IMAGE_SIZE: u32 = 128;

            shrink(data, MAX_IMAGE_SIZE)
        };
        self.id = LAST_IMAGE_ID.fetch_add(1, Ordering::SeqCst);
        self.url.clear();

        if !data.is_empty() {
            self.url.push_str("data:;base64,");
            base64::encode_config_buf(&data, STANDARD, &mut self.url);
        }
    }

    /// Checks if the image data is empty.
    pub fn is_empty(&self) -> bool {
        self.url.is_empty()
    }
}

/// With a Cached Image ID you can track image changes. It starts with an
/// uncached state and then gets updated with the images provided to it. It can
/// be reset at any point in order to force a change to be detected.
#[derive(Copy, Clone, PartialEq)]
pub enum CachedImageId {
    /// The initial uncached state.
    Uncached,
    /// The last image observed either was missing or contained no data.
    NoImage,
    /// The last image had actual data and the ID stored here.
    Image(usize),
}

impl Default for CachedImageId {
    fn default() -> Self {
        CachedImageId::Uncached
    }
}

impl CachedImageId {
    /// Updates the cached image ID based on the optional image provided to this
    /// method. If a change is observed the Data URL representing the image's
    /// data is returned. An empty string is returned when a transition to no
    /// image or no image data is observed.
    pub fn update_with<'i>(&mut self, image: Option<&'i Image>) -> Option<&'i str> {
        let new_value = image.map_or(CachedImageId::NoImage, |i| {
            if i.is_empty() {
                CachedImageId::NoImage
            } else {
                CachedImageId::Image(i.id())
            }
        });

        if *self != new_value {
            *self = new_value;
            Some(image.map_or("", |i| i.url()))
        } else {
            None
        }
    }

    /// Resets the state of the cached image ID to uncached.
    pub fn reset(&mut self) {
        *self = CachedImageId::Uncached;
    }
}
