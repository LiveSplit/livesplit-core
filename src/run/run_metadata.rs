use ordermap::{Iter, OrderMap};

#[derive(Default, Clone, Debug)]
pub struct RunMetadata {
    pub run_id: String,
    pub platform_name: String,
    pub uses_emulator: bool,
    pub region_name: String,
    variables: OrderMap<String, String>,
}

impl RunMetadata {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_variable<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.variables.insert(name.into(), value.into());
    }

    pub fn variables(&self) -> Iter<String, String> {
        self.variables.iter()
    }
}
