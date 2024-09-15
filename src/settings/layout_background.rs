use super::{Gradient, Image, ImageCache, ImageId};
use serde_derive::{Deserialize, Serialize};

/// The background of a layout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LayoutBackground<I = Image> {
    /// A gradient that describes the background coloration.
    Gradient(Gradient),
    /// An image that is stretched to fill the background. The stretch is meant
    /// to preserve the aspect ratio of the image, but always fill the full
    /// background.
    Image(BackgroundImage<I>),
}

/// An image that is stretched to fill the background. The stretch is meant to
/// preserve the aspect ratio of the image, but always fill the full background.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundImage<I> {
    /// The image itself.
    pub image: I,
    /// The brightness of the image in the range from `0` to `1`. This is for
    /// darkening the image if it's too bright.
    pub brightness: f32,
    /// The opacity of the image in the range from `0` to `1`. This is for
    /// making the image more transparent.
    pub opacity: f32,
    /// An additional gaussian blur that is applied to the image. It is in the
    /// range from `0` to `1` and is meant to be multiplied with the larger of
    /// the two dimensions of the image to ensure that the blur is independent
    /// of the resolution of the image and then multiplied by [`BLUR_FACTOR`] to scale it
    /// to a reasonable value. The resulting value is the sigma (standard
    /// deviation) of the gaussian blur.
    ///
    /// ```text
    /// sigma = BLUR_FACTOR * blur * max(width, height)
    /// ```
    pub blur: f32,
}

/// A constant that is part of the formula to calculate the sigma of a gaussian
/// blur for a [`BackgroundImage`]. Check its documentation for a deeper
/// explanation.
pub const BLUR_FACTOR: f32 = 0.05;

impl<I> BackgroundImage<I> {
    /// Changes the representation of the image, while retaining the other
    /// properties.
    pub const fn map<T>(&self, image: T) -> BackgroundImage<T> {
        BackgroundImage {
            image,
            brightness: self.brightness,
            opacity: self.opacity,
            blur: self.blur,
        }
    }
}

impl<I> Default for LayoutBackground<I> {
    fn default() -> Self {
        Self::Gradient(Default::default())
    }
}

impl LayoutBackground<Image> {
    /// Caches the background in the image cache provided and returns a
    /// [`LayoutBackground`] which contains either a gradient or a cached
    /// [`ImageId`](super::ImageId). If it is an image, the image gets marked as
    /// visited regardless of whether it was already in the cache or not.
    pub fn cache(&self, image_cache: &mut ImageCache) -> LayoutBackground<ImageId> {
        match self {
            LayoutBackground::Gradient(gradient) => LayoutBackground::Gradient(*gradient),
            LayoutBackground::Image(image) => LayoutBackground::Image(
                image.map(
                    *image_cache
                        .cache(image.image.id(), || image.image.clone())
                        .id(),
                ),
            ),
        }
    }
}

impl LayoutBackground<ImageId> {
    /// Converts the [`LayoutBackground`] containing an
    /// [`ImageId`](super::ImageId) into a [`LayoutBackground`] containing the
    /// image by possibly looking up the image in the image cache. This does not
    /// mark the image as visited.
    pub fn from_cache(self, image_cache: &ImageCache) -> LayoutBackground {
        match self {
            Self::Gradient(gradient) => LayoutBackground::Gradient(gradient),
            Self::Image(image) => LayoutBackground::Image(
                image.map(
                    image_cache
                        .lookup(&image.image)
                        .unwrap_or(Image::EMPTY)
                        .clone(),
                ),
            ),
        }
    }
}
