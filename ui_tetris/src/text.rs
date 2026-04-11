use graphics::types::{Color, FontSize, Scalar};

#[derive(Clone)]
pub struct Text {
    pub(super) center_x: Scalar,
    pub(super) center_y: Scalar,
    content: String,
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
        let mut content = String::from(text);
        content.push('|');
        Text {
            center_x,
            center_y,
            content,
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
        self.content.push('|');
    }

    pub fn push_str(&mut self, end: &str) {
        self.content.pop();
        self.content.push_str(end);
        self.content.push('|');
    }

    pub fn pop(&mut self) {
        self.content.pop();
        self.content.pop();
        self.content.push('|');
    }

    pub fn is_empty(&self) -> bool {
        self.content.eq("|")
    }

    pub fn get_editable_text(&self, cursor: bool) -> &str {
        &self.content[0..self.content.len().saturating_sub(usize::from(!cursor))]
    }

    pub fn get_text(&self) -> &str {
        &self.content[0..self.content.len().saturating_sub(1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_FONT_SIZE;
    use graphics::color::BLACK;
    use rstest::rstest;

    #[rstest]
    #[case("hello", "hello|")]
    #[case("", "|")]
    fn get_editable_text_is_correct(#[case] input: &str, #[case] expected: &str) {
        let text = Text::new(input, DEFAULT_FONT_SIZE, 0.0, 0.0, BLACK);

        assert_eq!(text.get_editable_text(false), input);
        assert_eq!(text.get_editable_text(true), expected);
    }
}
