use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub enum ColorType {
    RGBA(u8),
}
pub struct ImageBuffer<T, U = ()>(U, PhantomData<T>);
pub struct Rgba<T>(PhantomData<T>);

pub mod png {
    use super::ColorType;
    use super::super::png::Encoder;
    use std::io::Write;

    pub struct PNGEncoder<W>(W);

    impl<W: Write> PNGEncoder<W> {
        pub fn new(w: W) -> Self {
            PNGEncoder(w)
        }

        pub fn encode(&mut self, d: &[u8], w: u32, h: u32, _: ColorType) -> Result<(), ()> {
            Encoder::new(&mut self.0, w, h)
                .write_header()
                .map_err(|_| ())?
                .write_image_data(d)
                .map_err(|_| ())
        }
    }
}

impl<T, U> ImageBuffer<T, U> {
    pub fn from_raw(_: u32, _: u32, d: U) -> Option<ImageBuffer<Rgba<u8>, U>> {
        Some(ImageBuffer(d, PhantomData))
    }

    pub fn as_ref(&self) -> &[u8]
    where
        U: AsRef<[u8]>,
    {
        self.0.as_ref()
    }
}
