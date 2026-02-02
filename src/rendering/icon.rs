use crate::settings::{HasImageId, ImageId};

use super::{SharedOwnership, resource::Handle};

pub struct CachedImage<T> {
    pub id: ImageId,
    pub image: Option<ImageHandle<T>>,
}

// FIXME: Is this still useful?
pub struct ImageHandle<T> {
    pub handle: Handle<T>,
}

impl<T: SharedOwnership> SharedOwnership for ImageHandle<T> {
    fn share(&self) -> Self {
        Self {
            handle: self.handle.share(),
        }
    }
}

impl<T> HasImageId for CachedImage<T> {
    fn image_id(&self) -> &ImageId {
        &self.id
    }
}
