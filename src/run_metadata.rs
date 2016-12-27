#[derive(Default, Clone, Debug)]
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
    pub fn set_run_id<S>(&mut self, id: S)
        where S: AsRef<str>
    {
        self.run_id.clear();
        self.run_id.push_str(id.as_ref());
    }
}
