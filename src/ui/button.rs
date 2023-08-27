use crate::{settings::DEFAULT_FONT_SIZE, ui::text::Text};
use graphics::color;
use piston::MouseButton;

#[derive(Clone)]
pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Text,
    pub background_color: graphics::types::Color,
    commit: bool,
    is_pressed: bool,
}

impl Button {
    pub fn new(x: f64, y: f64, width: f64, height: f64, text: &str) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: Text::new(text, DEFAULT_FONT_SIZE, 0.0, 0.0, color::BLACK),
            background_color: [0.8, 0.8, 0.8, 1.0],
            commit: false,
            is_pressed: false,
        }
    }

    pub fn default() -> Button {
        Button {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text: Text::default(),
            background_color: [0.0, 0.0, 0.0, 0.0],
            commit: false,
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

    pub fn commit(&mut self) -> bool {
        if self.commit {
            self.commit = false;
            true
        } else {
            false
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.are_coords_inside_button(cursor_position[0], cursor_position[1]) {
                    self.background_color = [0.5, 0.5, 0.5, 1.0];
                    self.is_pressed = true;
                    self.commit = true;
                } else {
                    self.is_pressed = false;
                }
            }
            _ => {}
        };
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.background_color = [0.8, 0.8, 0.8, 1.0];
                self.is_pressed = false;
            }
            _ => {}
        }
    }
}
