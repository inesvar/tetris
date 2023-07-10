use graphics::color;
use piston::MouseButton;

use crate::ui::text::Text;

pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Text,
    pub background_color: graphics::types::Color,

    is_pressed: bool,
}

impl Button {
    pub fn new(x: f64, y: f64, width: f64, height: f64, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: Text::new(text, 16, 0.0, 0.0, color::BLACK),
            background_color: [0.8, 0.8, 0.8, 1.0],

            is_pressed: false,
        }
    }

    pub fn are_coords_inside_button(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0
            && x <= self.x + self.width / 2.0
            && y >= self.y - self.height / 2.0
            && y <= self.y + self.height / 2.0
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.are_coords_inside_button(cursor_position[0], cursor_position[1]) {
                    self.background_color = [0.5, 0.5, 0.5, 1.0];
                    self.is_pressed = true;
                } else {
                    self.is_pressed = false;
                }
            }
            _ => {}
        };
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                self.background_color = [0.8, 0.8, 0.8, 1.0];
                self.is_pressed = false;
            }
            _ => {}
        }
    }
}
