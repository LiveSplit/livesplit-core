use core::fmt::Debug;
use livesplit_auto_splitting::settings;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AutoSplitterSettings {
    pub custom_settings: settings::Map,
}

impl AutoSplitterSettings {
    pub fn set_custom_settings(&mut self, custom_settings: settings::Map) {
        self.custom_settings = custom_settings;
    }
}
