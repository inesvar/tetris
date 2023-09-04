extern crate clipboard;
extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::{
    app::App,
    assets::Assets,
    settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OPENGL_VERSION},
};
use piston::{
    event_loop::{EventSettings, Events},
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
    AdvancedWindow, {Button, MouseCursorEvent, PressEvent, ReleaseEvent, TextEvent},
};
use piston_window::PistonWindow;
use sdl2_window::Sdl2Window;

mod app;
mod assets;
mod macros;
mod settings;
mod ui;

#[derive(PartialEq, Debug)]
pub enum PlayerConfig {
    Local,
    TwoLocal,
    TwoRemote { local_ip: String, remote_ip: String },
    Viewer(String),
}

impl PlayerConfig {
    pub fn is_remote(&self) -> bool {
        match self {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } => true,
            PlayerConfig::Viewer(_) => true,
            _ => false,
        }
    }

    pub fn is_multiplayer(&self) -> bool {
        match self {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } => true,
            PlayerConfig::TwoLocal => true,
            _ => false,
        }
    }
}

fn main() {
    // Create a Sdl2 window.
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(OPENGL_VERSION);
    let mut fall_speed_divide: u64 = 50;
    let mut freeze: u64 = 50;
    let mut multiplayer = false;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args, fall_speed_divide, freeze);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }

        if app.player_config.is_remote() {
            once!("main knows that we're remote");
            app.handle_remote();
        }

        if app.player_config.is_multiplayer() {
            if !multiplayer {
                window.set_size([DEFAULT_WINDOW_WIDTH * 2, DEFAULT_WINDOW_HEIGHT]);
                multiplayer = true;
            }
        } else if multiplayer {
            window.set_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT]);
            multiplayer = false;
        }

        if let Some(text) = e.text_args() {
            app.handle_text_input(&text);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }

        // TODO : think about this, move this and make it less painful
        match app.clock {
            i if i <= 30.0 => {
                fall_speed_divide = 50;
                freeze = 50
            }
            i if i <= 60.0 => {
                fall_speed_divide = 40;
                freeze = 50
            }
            i if i <= 90.0 => {
                fall_speed_divide = 30;
                freeze = 50
            }
            i if i <= 120.0 => {
                fall_speed_divide = 20;
                freeze = 50
            }
            _ => {
                fall_speed_divide = 15;
                freeze = 50
            }
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            app.handle_mouse_press(button);
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            app.handle_mouse_release(button);
        }

        e.mouse_cursor(|cursor_position| {
            app.cursor_position = cursor_position;
        });
    }
}
