use super::text::Text;
use super::Rectangle;
use super::DEFAULT_FONT_SIZE;
use graphics::color;
use graphics::types::{Color, Scalar};

/// Button that changes color when pressed.
/// The widget manager will know the button was pressed when it makes its next query.
#[derive(Clone)]
pub struct Button {
    pub(super) rect: Rectangle,
    pub(super) text: Text,
    color: Color,
    color_when_pressed: Color,
    has_been_pressed: bool,
    is_pressed: bool,
}

impl Button {
    pub(super) fn new(
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
        text: &str,
    ) -> Self {
        Button {
            rect: Rectangle::new(center_x, center_y, width, height),
            text: Text::new(text, DEFAULT_FONT_SIZE, 0.0, 0.0, color::BLACK),
            color: [0.8, 0.8, 0.8, 1.0],
            color_when_pressed: [0.5, 0.5, 0.5, 1.0],
            has_been_pressed: false,
            is_pressed: false,
        }
    }

    pub(super) fn new_pressed(
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
        text: &str,
    ) -> Self {
        Button {
            rect: Rectangle::new(center_x, center_y, width, height),
            text: Text::new(text, DEFAULT_FONT_SIZE, 0.0, 0.0, color::BLACK),
            color: [0.8, 0.8, 0.8, 1.0],
            color_when_pressed: [0.5, 0.5, 0.5, 1.0],
            has_been_pressed: true,
            is_pressed: false,
        }
    }

    pub(super) fn has_been_pressed(&mut self) -> bool {
        let has_been_pressed = self.has_been_pressed;
        self.has_been_pressed = false;
        has_been_pressed
    }

    pub(super) fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        if self.rect.contains(cursor_position) {
            println!("button press");
            self.is_pressed = true;
            self.has_been_pressed = true;
        };
    }

    pub(super) fn handle_left_click_release(&mut self) {
        self.is_pressed = false;
    }

    pub(super) fn color(&self) -> Color {
        if self.is_pressed {
            self.color_when_pressed
        } else {
            self.color
        }
    }
}
