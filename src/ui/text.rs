use graphics::types::Color;

pub struct Text {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub font_size: u32,
    pub color: Color,
    pub(crate) view: graphics::Text,
}

impl Text {
    pub fn new(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text {
            x,
            y,
            text: String::from(text),
            font_size,
            color,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
