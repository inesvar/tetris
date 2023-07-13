use crate::{settings::TEXT_COLOR, ui::text::Text};
use graphics::color;
use piston::{Key, MouseButton};

use super::keybindings::keys_to_string;

pub struct KeyInput {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) keys: Vec<Key>,
    pub(in crate::ui) text: Text,
    pub(in crate::ui) cursor: String,
    pub(in crate::ui) info_text: Text,
    placeholder: String,
    is_focused: bool,
    pub(in crate::ui) animation_counter: u64,
}

impl KeyInput {
    pub fn new_with_info(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        keys: &[Key],
        info_text: &str,
    ) -> Self {
        let placeholder = &keys_to_string(keys);
        let mut vec_keys = vec![];
        for key in keys {
            vec_keys.push(*key);
        }
        KeyInput {
            x,
            y,
            width,
            height,
            keys: vec_keys,
            cursor: String::from(""),
            info_text: Text::new(info_text, 16, x, y, TEXT_COLOR),
            text: Text::new(placeholder, 16, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            is_focused: false,
            animation_counter: 0,
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
                    if self.text.content == self.placeholder {
                        self.text.set_text(String::from(""));
                    }
                } else {
                    self.is_focused = false;
                    if self.text.content == "" {
                        self.text.set_text(String::from(&self.placeholder));
                    }
                }
            }
            _ => {}
        };
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if self.is_focused {
            match key {
                Key::Backspace => {
                    println!("before first pop, text is : {}.", self.text.content);
                    self.text.content.pop();
                    let mut last_word = true;
                    while self.text.content.len() > 0 && last_word {
                        println!("entered deleting loop, text is : {}.", self.text.content);
                        match self
                            .text
                            .content
                            .get((self.text.content.len() - 1)..=(self.text.content.len() - 1))
                        {
                            Some(" ") => {
                                last_word = false;
                                break;
                            }
                            _ => {}
                        }
                        self.text.content.pop();
                    }
                }
                Key::Return => {
                    self.is_focused = false;
                    if self.text.content == "" {
                        self.text.set_text(String::from(&self.placeholder));
                    }
                }
                Key::Down => {
                    if self.is_focused {
                        self.text.content.push_str("Down, ");
                    }
                }
                Key::Space => {
                    if self.is_focused {
                        self.text.content.push_str("Space, ");
                    }
                }
                Key::Right => {
                    if self.is_focused {
                        self.text.content.push_str("Right, ");
                    }
                }
                Key::Left => {
                    if self.is_focused {
                        self.text.content.push_str("Left, ");
                    }
                }
                Key::Up => {
                    if self.is_focused {
                        self.text.content.push_str("Up, ");
                    }
                }
                Key::NumPad0 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad0, ");
                    }
                }
                Key::NumPad1 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad1, ");
                    }
                }
                Key::NumPad2 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad2, ");
                    }
                }
                Key::NumPad3 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad3, ");
                    }
                }
                Key::NumPad4 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad4, ");
                    }
                }
                Key::NumPad5 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad5, ");
                    }
                }
                Key::NumPad6 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad6, ");
                    }
                }
                Key::NumPad7 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad7, ");
                    }
                }
                Key::NumPad8 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad8, ");
                    }
                }
                Key::NumPad9 => {
                    if self.is_focused {
                        self.text.content.push_str("Numpad9, ");
                    }
                }
                _ => {}
            }
        }
    }

    pub fn handle_text_input(&mut self, text: &String) {
        if self.is_focused {
            if text == "" {
                println!("nothing");
                return;
            }
            if text == "," {
                self.text.content.push_str(&(text.to_owned() + " "));
            } else if text != " " && !text.chars().any(char::is_numeric) && text != "" {
                println!("normal text : {}.", text);
                self.text.content.push_str(&(text.to_owned() + ", "));
            }
        }
    }

    pub fn get_focused(&self) -> bool {
        self.is_focused
    }
}
