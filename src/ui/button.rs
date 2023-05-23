use graphics::{color};
use piston::MouseButton;


use crate::ui::text::Text;

pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Text,
    pub pressed: bool,
    pub background_color: graphics::types::Color
}

impl Button {
    pub fn new(x: f64, y: f64, width: f64, height: f64, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: Text::new(text, 16, 0.0, 0.0, color::BLACK),
            pressed: false,
            background_color: [0.8, 0.8, 0.8, 1.0]
        }
    }

    pub fn is_clicked(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0 && x <= self.x + self.width / 2.0 && y >= self.y - self.height / 2.0 && y <= self.y + self.height / 2.0
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.is_clicked(cursor_position[0], cursor_position[1]) {
                    self.background_color = [0.5, 0.5, 0.5, 1.0];
                }
            },
            _ => {}
        };
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.is_clicked(cursor_position[0], cursor_position[1]) {
                    self.background_color = [0.8, 0.8, 0.8, 1.0];
                }
            },
            _ => {}
        }
    }
}
