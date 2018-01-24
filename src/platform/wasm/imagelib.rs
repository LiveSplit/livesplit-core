use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub enum ColorType {
    RGBA(u8),
}
pub struct ImageBuffer<T, U = ()>(PhantomData<(T, U)>);
pub struct Rgba<T>(PhantomData<T>);

pub mod png {
    use super::ColorType;

    pub struct PNGEncoder;

    impl PNGEncoder {
        pub fn new<T>(_: T) -> Self {
            PNGEncoder
        }

        pub fn encode(&self, _: &[u8], _: u32, _: u32, _: ColorType) -> Result<(), ()> {
            Ok(())
        }
    }
}

impl<T, U> ImageBuffer<T, U> {
    pub fn from_raw(_: u32, _: u32, _: U) -> Option<ImageBuffer<Rgba<u8>, U>> {
        Some(ImageBuffer(PhantomData))
    }

    pub fn as_ref(&self) -> &'static [u8] {
        &[]
    }
}
