use super::{button::Button, key_input::KeyInput, text::Text, text_input::TextInput};
use super::{
    DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH,
    GUEST_PORT, HOST_PORT,
};
use crate::keybindings::Keybindings;
use arboard::Clipboard;
use core_tetris::TetrisCommand;
use graphics::math::Scalar;
use local_ip_address::local_ip;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ButtonType {
    ViewChange(ViewChangeButton),
    CurrentView(CurrentViewButton),
}

impl From<ViewChangeButton> for ButtonType {
    fn from(value: ViewChangeButton) -> Self {
        ButtonType::ViewChange(value)
    }
}

impl From<CurrentViewButton> for ButtonType {
    fn from(value: CurrentViewButton) -> Self {
        ButtonType::CurrentView(value)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ViewChangeButton {
    ToSinglePlayerGame,
    ToTwoRemoteGameInfo { local_ip: String, remote_ip: String },
    ToCreateRoom,
    ToJoinRoom,
    ToSettings,
    BackToMainMenu,
    BackToGame,
    ToTwoLocalGame,
    ToPause, // TODO: should be in the next enum
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum CurrentViewButton {
    CopyToClipboard,
    PasteFromClipboard,
    ToTwoRemoteGame, // TODO: that's weird...
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TextInputType {
    #[allow(unused)]
    DebugTextInput,
    IpAddressInput,
}

#[derive(Hash, PartialEq, Eq)]
pub enum TextType {
    Title,
    Restart,
    Pause,
    Timer,
}

pub struct InteractiveWidgetManager {
    pub(super) buttons: HashMap<ButtonType, Button>,
    pub(super) text_inputs: HashMap<TextInputType, TextInput>,
    pub(super) key_inputs: HashMap<(TetrisCommand, usize), KeyInput>,
    pub(super) texts: HashMap<TextType, Text>,
}

impl InteractiveWidgetManager {
    pub fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_left_click(cursor_position);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_left_click(cursor_position);
        }

        for button in self.buttons.values_mut() {
            button.handle_left_click(cursor_position);
        }
    }

    pub fn handle_left_click_release(&mut self) {
        for button in self.buttons.values_mut() {
            button.handle_left_click_release();
        }
    }

    pub fn handle_key_press(&mut self, key: piston::Key) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_key_press(key);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_key_press(key);
        }
    }

    pub fn handle_text_input(&mut self, text: &str) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_text_input(text);
        }
    }

    pub fn get_button(&mut self, button_type: &ButtonType) -> &mut Button {
        self.buttons
            .get_mut(button_type)
            .unwrap_or_else(|| panic!("Button {:?} not found", button_type))
    }

    pub fn get_input(&mut self, input_type: TextInputType) -> &mut TextInput {
        self.text_inputs
            .get_mut(&input_type)
            .unwrap_or_else(|| panic!("Input {:?} not found", input_type))
    }

    pub fn get_new_keybindings(&mut self, player: usize) -> Keybindings {
        let mut keybindings_manager = Keybindings::default();
        for ((command, id), key_input) in self.key_inputs.iter_mut() {
            if *id != player {
                continue;
            }
            if key_input.keys.is_empty() {
                keybindings_manager.set_keys(*command, key_input.initial_keys.clone());
            } else {
                keybindings_manager.set_keys(*command, key_input.keys.clone());
            }
        }
        keybindings_manager
    }

    pub fn set_text(&mut self, text_type: TextType, new_text: String) {
        if let Some(text) = self.texts.get_mut(&text_type) {
            text.set_text(new_text);
        }
    }

    pub fn update_clipboard(&mut self) {
        if let Some(button) = self
            .buttons
            .get_mut(&CurrentViewButton::CopyToClipboard.into())
        {
            if button.has_been_pressed() {
                let ip = local_ip().unwrap().to_string();
                //let ip = "127.0.0.1".to_string();
                let text = format!("{}{}", ip, HOST_PORT);
                let mut clipboard = Clipboard::new().expect("Clipboard is not supported");
                clipboard
                    .set_text(text)
                    .expect("Setting the clipboard failed");
            }
        };
        if let Some(button) = self
            .buttons
            .get_mut(&CurrentViewButton::PasteFromClipboard.into())
        {
            if button.has_been_pressed() {
                let mut clipboard = Clipboard::new().expect("Clipboard is not supported");
                let ip = clipboard.get_text().expect("Getting the clipboard failed");
                let text_input = self.get_input(TextInputType::IpAddressInput);
                text_input.text.set_text(ip);
            }
        };
    }

    pub fn update_from_text(&mut self) {
        let button = self.get_button(&CurrentViewButton::ToTwoRemoteGame.into());
        if button.has_been_pressed() {
            let text_input = self.get_input(TextInputType::IpAddressInput);
            let remote_ip = String::from(text_input.text.get_text());
            println!("remote ip is {remote_ip}");
            let local_ip = local_ip().unwrap().to_string();
            //let local_ip = "127.0.0.1".to_string();
            let local_ip = format!("{}{}", local_ip, GUEST_PORT);

            let join_room = Button::new_pressed(
                DEFAULT_WINDOW_WIDTH / 2.0,
                DEFAULT_WINDOW_HEIGHT / 2.0,
                DEFAULT_BUTTON_WIDTH,
                DEFAULT_BUTTON_HEIGHT,
                "Join this room",
            );

            self.buttons.insert(
                ViewChangeButton::ToTwoRemoteGameInfo {
                    local_ip,
                    remote_ip,
                }
                .into(),
                join_room,
            );
        }
    }

    pub fn update_view(&mut self) -> Option<ViewChangeButton> {
        for (button_type, button) in self.buttons.iter_mut() {
            if let ButtonType::ViewChange(view_change_button) = button_type {
                if button.has_been_pressed() {
                    println!("button type is {:?}", button_type);
                    return Some(view_change_button.clone());
                }
            }
        }
        None
    }
}
