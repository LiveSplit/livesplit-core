use super::Field;

#[derive(Default, Serialize, Deserialize)]
pub struct SettingsDescription {
    pub fields: Vec<Field>,
}

impl SettingsDescription {
    pub fn with_fields(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}
