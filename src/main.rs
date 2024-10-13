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

/// Creates the window and the application, runs the event loop.
fn main() {
    // Create a Glfw window.
    let mut window: PistonWindow<GlfwWindow> =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .build()
            .unwrap();
    let mut bigger_window_size = false;

    // Create the app.
    let mut app = App::new(OPENGL_VERSION);

    // Start the event loop.
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // Handle the local user(s) input.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }

        if let Some(text) = e.text_args() {
            app.handle_text_input(&text);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            app.handle_mouse_press(button);
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            app.handle_mouse_release(button);
        }

        if let Some(cursor_position) = e.mouse_cursor_args() {
            app.cursor_position = cursor_position;
        }

        // Update the size of the window if necessary.
        if app.player_config.is_multiplayer() {
            if !bigger_window_size {
                window.set_size([DEFAULT_WINDOW_WIDTH * 2, DEFAULT_WINDOW_HEIGHT]);
                bigger_window_size = true;
            }
        } else if bigger_window_size {
            window.set_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT]);
            bigger_window_size = false;
        }
        
        // Handle the optional remote user input.
        if app.player_config.is_remote() {
            once!("main knows that we're remote");
            app.handle_remote();
        }

        // Finally update the application and render to the screen.
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
