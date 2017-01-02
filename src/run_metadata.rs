use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct RunMetadata {
    run_id: String,
    platform_name: String,
    uses_emulator: bool,
    region_name: String,
    variables: HashMap<String, String>,
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

    #[inline]
    pub fn set_platform_name<S>(&mut self, name: S)
        where S: AsRef<str>
    {
        self.platform_name.clear();
        self.platform_name.push_str(name.as_ref());
    }

    #[inline]
    pub fn set_emulator_usage(&mut self, uses_emulator: bool) {
        self.uses_emulator = uses_emulator;
    }

    #[inline]
    pub fn set_region_name<S>(&mut self, region_name: S)
        where S: AsRef<str>
    {
        self.region_name.clear();
        self.region_name.push_str(region_name.as_ref());
    }

    pub fn add_variable<N, V>(&mut self, name: N, value: V)
        where N: Into<String>,
              V: Into<String>
    {
        self.variables.insert(name.into(), value.into());
    }
}
