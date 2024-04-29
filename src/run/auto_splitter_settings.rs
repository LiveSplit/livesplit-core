use crate::run::parser::livesplit::Version;
use core::fmt::Debug;
use livesplit_auto_splitting::settings;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AutoSplitterSettings {
    pub version: Version,
    pub script_path: String,
    pub custom_settings: settings::Map,
}

impl AutoSplitterSettings {
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn set_script_path(&mut self, script_path: String) {
        self.script_path = script_path;
    }

    pub fn set_custom_settings(&mut self, custom_settings: settings::Map) {
        self.custom_settings = custom_settings;
    }
}
