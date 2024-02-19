use crate::settings::{HasImageId, ImageId};

use super::{resource::Handle, SharedOwnership};

pub struct CachedImage<T> {
    pub id: ImageId,
    pub image: Option<ImageHandle<T>>,
}

pub struct ImageHandle<T> {
    pub handle: Handle<T>,
    pub aspect_ratio: f32,
}

impl<T: SharedOwnership> SharedOwnership for ImageHandle<T> {
    fn share(&self) -> Self {
        Self {
            handle: self.handle.share(),
            aspect_ratio: self.aspect_ratio,
        }
    }
}

impl<T> HasImageId for CachedImage<T> {
    fn image_id(&self) -> &ImageId {
        &self.id
    }
}
