use graphics::types::{Color, FontSize, Scalar};

#[derive(Clone)]
pub struct Text {
    pub(super) center_x: Scalar,
    pub(super) center_y: Scalar,
    pub(super) content: String,
    pub(super) use_tetris_font: bool,
    pub(crate) view: graphics::Text,
}

impl Text {
    fn inner_new(
        text: &str,
        use_tetris_font: bool,
        font_size: FontSize,
        center_x: Scalar,
        center_y: Scalar,
        color: Color,
    ) -> Text {
        Text {
            center_x,
            center_y,
            content: String::from(text),
            use_tetris_font,
            view: graphics::text::Text::new_color(color, font_size),
        }
    }

    pub fn new(text: &str, font_size: FontSize, x: Scalar, y: Scalar, color: Color) -> Text {
        Text::inner_new(text, false, font_size, x, y, color)
    }

    pub fn new_with_tetris_font(
        text: &str,
        font_size: FontSize,
        x: Scalar,
        y: Scalar,
        color: Color,
    ) -> Text {
        Text::inner_new(text, true, font_size, x, y, color)
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
    }
}
