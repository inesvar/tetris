use graphics::color;
use piston::MouseButton;
use crate::ui::text::Text;

pub struct TextInput {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(in crate::ui) text: Text,
    placeholder: String,
    is_focused: bool,
}

impl TextInput {
    pub fn new(x: f64, y: f64, width: f64, height: f64, placeholder: &str) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            text: Text::new(placeholder, 16, x, y, color::WHITE),
            placeholder: String::from(placeholder),
            is_focused: false,
        }
    }

    pub fn are_coords_inside_input(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0
            && x <= self.x + self.width / 2.0
            && y >= self.y - self.height / 2.0
            && y <= self.y + self.height / 2.0
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.are_coords_inside_input(cursor_position[0], cursor_position[1]) {
                    self.is_focused = true;
                    if self.text.text == self.placeholder {
                        self.text.set_text(String::from("|"));
                    }
                } else {
                    self.is_focused = false;
                    if self.text.text == "|" {
                        self.text.set_text(String::from(&self.placeholder));
                    }
                }
            }
            _ => {}
        };
    }

    pub fn get_focused(&self) -> bool {
        self.is_focused
    }
}