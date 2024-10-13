//! A Tetris application for one or two players.
//!
//! It's possible to play either locally or remotely.
//! The keybindings are customizable.
extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::{
    app::App,
    assets::Assets,
    settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OPENGL_VERSION},
};
use glfw_window::GlfwWindow;
use piston::{
    event_loop::{EventSettings, Events},
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
    AdvancedWindow, {Button, MouseCursorEvent, PressEvent, ReleaseEvent, TextEvent},
};
use piston_window::PistonWindow;

mod app;
mod assets;
mod settings;
mod ui;
mod utils;

/// Creates a window and an event loop interacting with the Tetris application.
fn main() {
    // Create a Sdl2 window.
    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(OPENGL_VERSION);
    // TODO: having those variables here is unelegant
    // they should be in app and their value in settings.
    let mut fall_speed_divide: u64 = 50;
    let mut freeze: u64 = 50;
    let mut multiplayer = false; // TODO: rename window_size

    // Start event loop
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

        // TODO : this should be in app :)
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

        // TODO: use if let style to be consistent
        e.mouse_cursor(|cursor_position| {
            app.cursor_position = cursor_position;
        });
    }
}
