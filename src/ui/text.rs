use graphics::types::Color;

pub struct Text {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub font_size: u32,
    pub color: Color,
}

impl Text {
    pub fn new(text: String, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text { x, y, text, font_size, color }
    }
}