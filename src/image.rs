//! Images are stored as URLs. That often means they are encoded as Data-URLs, but that may not
//! always be the case. Each image object comes with an ID that changes whenever the image is
//! modified. IDs are unique across different images. ID 0 is never used so it can be used as an
//! initial state to refresh state at the beginning.

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use base64::{self, STANDARD};

static LAST_IMAGE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Debug, Clone)]
pub struct Image {
    url: String,
    id: usize,
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
    pub fn new(data: &[u8]) -> Self {
        let mut image = Image {
            url: String::new(),
            id: 0,
        };
        image.modify(data);
        image
    }

    pub fn from_file<P, B>(path: P, mut buf: B) -> io::Result<Image>
    where
        P: AsRef<Path>,
        B: AsMut<Vec<u8>>,
    {
        let buf = buf.as_mut();
        buf.clear();
        BufReader::new(File::open(path)?).read_to_end(buf)?;
        Ok(Image::new(buf))
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[inline]
    pub fn check_for_change(&self, old_id: &mut usize) -> Option<&str> {
        if *old_id != self.id {
            *old_id = self.id;
            Some(self.url())
        } else {
            None
        }
    }

    pub fn modify(&mut self, data: &[u8]) {
        self.id = LAST_IMAGE_ID.fetch_add(1, Ordering::SeqCst) + 1;
        self.url.clear();

        if !data.is_empty() {
            self.url.push_str("data:;base64,");
            base64::encode_config_buf(data, STANDARD, &mut self.url);
        }
    }
}
