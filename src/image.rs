use base64::{self, STANDARD};
use image_shrink::shrink;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static LAST_IMAGE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

const MAX_IMAGE_SIZE: u32 = 128;

/// Images can be used to store segment and game icons. Each image object comes
/// with an ID that changes whenever the image is modified. IDs are unique
/// across different images. ID 0 is never used so it can be used as an initial
/// state to refresh state at the beginning. You can query the image's data as a
/// Data URL. There's no specific image format you need to use for the images.
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

    /// Checks whether the image has changed by providing the last known image
    /// ID. You can use 0 as an initial value for this image ID. If the image
    /// changed, the new ID is stored in the ID you provided and the image's URL
    /// is returned.
    #[inline]
    pub fn check_for_change(&self, old_id: &mut usize) -> Option<&str> {
        if *old_id != self.id {
            *old_id = self.id;
            Some(self.url())
        } else {
            None
        }
    }

    /// Modifies an image by replacing its image data with the new image data
    /// provided. The image's ID changes to a new unique ID.
    pub fn modify(&mut self, data: &[u8]) {
        let data = shrink(data, MAX_IMAGE_SIZE);
        self.id = LAST_IMAGE_ID.fetch_add(1, Ordering::SeqCst) + 1;
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
