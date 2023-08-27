use graphics::types::Color;

use crate::settings::DEFAULT_FONT_SIZE;

#[derive(Clone)]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub font_size: u32,
    pub color: Color,
    pub(crate) view: graphics::Text,
}

impl Text {
    pub fn new(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text {
            x,
            y,
            content: String::from(text),
            font_size,
            color,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn default() -> Text {
        Text {
            x: 0.0,
            y: 0.0,
            content: String::from(""),
            font_size: DEFAULT_FONT_SIZE,
            color: [0.0, 0.0, 0.0, 0.0],
            view: graphics::text::Text::new_color([0.0, 0.0, 0.0, 0.0], DEFAULT_FONT_SIZE),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}
