use super::Color;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Gradient {
    Transparent,
    Plain(Color),
    Vertical(Color, Color),
    Horizontal(Color, Color),
}
