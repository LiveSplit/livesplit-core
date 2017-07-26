use super::Value;

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub text: String,
    pub value: Value,
}

impl Field {
    pub fn new(text: String, value: Value) -> Self {
        Self { text, value }
    }
}
