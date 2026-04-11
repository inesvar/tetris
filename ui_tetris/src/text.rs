#[allow(unused)]
use super::DEFAULT_FONT_SIZE;
use graphics::types::Color;

#[derive(Clone)]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub(super) content: String,
    pub use_tetris_font: bool,
    pub(crate) view: graphics::Text,
}

impl Text {
    pub fn new(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text {
            x,
            y,
            content: String::from(text),
            use_tetris_font: false,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn new_with_tetris_font(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text {
            x,
            y,
            content: String::from(text),
            use_tetris_font: true,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}
