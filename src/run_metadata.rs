#[derive(Default, Clone)]
pub struct RunMetadata {
    run_id: String,
    platform_name: String,
    uses_emulator: bool,
    region_name: String,
    variables: Vec<(String, String)>,
}

impl RunMetadata {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn set_run_id(&mut self, id: String) {
        self.run_id = id;
    }
}
