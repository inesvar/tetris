use graphics::types::Color;

#[derive(Clone)]
pub struct Text {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) content: String,
    pub(super) use_tetris_font: bool,
    pub(crate) view: graphics::Text,
}

impl Text {
    fn inner_new(
        text: &str,
        use_tetris_font: bool,
        font_size: u32,
        x: f64,
        y: f64,
        color: Color,
    ) -> Text {
        Text {
            x,
            y,
            content: String::from(text),
            use_tetris_font,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn new(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text::inner_new(text, false, font_size, x, y, color)
    }

    pub fn new_with_tetris_font(text: &str, font_size: u32, x: f64, y: f64, color: Color) -> Text {
        Text::inner_new(text, true, font_size, x, y, color)
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}
