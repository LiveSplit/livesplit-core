use crate::run::parser::livesplit::Version;
use core::fmt::Debug;
use livesplit_auto_splitting::settings;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AutoSplitterSettings {
    pub version: Version,
    pub script_path: String,
    pub custom_settings: Vec<CustomSetting>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomSetting {
    pub id: String,
    pub setting_type: settings::WidgetKind,
    pub value: settings::Value,
}

impl AutoSplitterSettings {
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn set_script_path(&mut self, script_path: String) {
        self.script_path = script_path;
    }

    pub fn add_custom_setting(&mut self, custom_setting: CustomSetting) {
        self.custom_settings.push(custom_setting);
    }
}

impl CustomSetting {
    pub fn new() -> Self {
        Self {
            id: String::default(),
            setting_type: settings::WidgetKind::Bool {
                default_value: false,
            },
            value: settings::Value::Bool(false),
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_setting_type(&mut self, setting_type: settings::WidgetKind) {
        self.setting_type = setting_type;
    }

    pub fn set_value(&mut self, value: settings::Value) {
        self.value = value
    }
}
