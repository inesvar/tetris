use super::text::Text;
use super::DEFAULT_FONT_SIZE;
use graphics::color;
use graphics::types::Color;

/// Button that changes color when pressed.
/// The widget manager will know the button was pressed when it makes its next query.
#[derive(Clone)]
pub struct Button {
    pub center_x: f64,
    pub center_y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Text,
    color: Color,
    color_when_pressed: Color,
    has_been_pressed: bool,
    is_pressed: bool,
}

impl Button {
    pub fn new(center_x: f64, center_y: f64, width: f64, height: f64, text: &str) -> Self {
        Button {
            center_x,
            center_y,
            width,
            height,
            text: Text::new(text, DEFAULT_FONT_SIZE, 0.0, 0.0, color::BLACK),
            color: [0.8, 0.8, 0.8, 1.0],
            color_when_pressed: [0.5, 0.5, 0.5, 1.0],
            has_been_pressed: false,
            is_pressed: false,
        }
    }

    pub fn new_pressed(center_x: f64, center_y: f64, width: f64, height: f64, text: &str) -> Self {
        Button {
            center_x,
            center_y,
            width,
            height,
            text: Text::new(text, DEFAULT_FONT_SIZE, 0.0, 0.0, color::BLACK),
            color: [0.8, 0.8, 0.8, 1.0],
            color_when_pressed: [0.5, 0.5, 0.5, 1.0],
            has_been_pressed: true,
            is_pressed: false,
        }
    }

    pub fn are_coords_inside_button(&self, &[x, y]: &[f64; 2]) -> bool {
        x >= self.center_x - self.width / 2.0
            && x <= self.center_x + self.width / 2.0
            && y >= self.center_y - self.height / 2.0
            && y <= self.center_y + self.height / 2.0
    }

    pub fn has_been_pressed(&mut self) -> bool {
        let has_been_pressed = self.has_been_pressed;
        self.has_been_pressed = false;
        has_been_pressed
    }

    pub fn handle_left_click(&mut self, cursor_position: &[f64; 2]) {
        if self.are_coords_inside_button(cursor_position) {
            println!("button press");
            self.is_pressed = true;
            self.has_been_pressed = true;
        };
    }

    pub fn handle_left_click_release(&mut self) {
        self.is_pressed = false;
    }

    pub fn color(&self) -> Color {
        if self.is_pressed {
            self.color_when_pressed
        } else {
            self.color
        }
    }
}
