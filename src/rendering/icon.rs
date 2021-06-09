use super::resource::Handle;

pub struct Icon<T> {
    pub image: Handle<T>,
    pub aspect_ratio: f32,
}
