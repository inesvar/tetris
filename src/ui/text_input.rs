use graphics::color;
use crate::ui::text::Text;

pub struct TextInput {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pub(in crate::ui) text: Text,
    placeholder: String,
}

impl TextInput {
    pub fn new(x: f64, y: f64, placeholder: &str) -> Self {
        TextInput {
            x,
            y,
            width: 0.0,
            height: 0.0,
            text: Text::new(placeholder, 16, x, y, color::WHITE),
            placeholder: String::from(placeholder),
        }
    }
}