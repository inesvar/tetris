extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::app::App;
use crate::assets::Assets;
use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OPENGL_VERSION};
use app::PlayerConfig;
use clap::Parser;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, MouseCursorEvent, PressEvent, ReleaseEvent, TextEvent};

mod app;
mod assets;
mod circular_buffer;
mod keyboard;
mod local_player;
mod macros;
mod player_screen;
mod remote_player;
mod settings;
mod tetris_back_end;
mod ui;

#[derive(Parser, Debug)]
struct Args {
    // two local players
    #[arg(long)]
    two_local: bool,
    // two remote player
    #[arg(long)]
    two_remote: bool,
    // sending screen
    #[arg(short, long)]
    streamer: bool,
    // viewing remote screen
    #[arg(short, long)]
    viewer: bool,
}

// TO CHECK OUT THE COMMAND LINE OPTIONS use the following template
// cargo run -- -h

fn main() {
    // Check the command line arguments.
    let args = Args::parse();

    let config: PlayerConfig = if args.two_local {
        PlayerConfig::TwoLocal
    } else if args.two_remote {
        PlayerConfig::TwoRemote
    } else if args.streamer {
        PlayerConfig::Streamer
    } else if args.viewer {
        PlayerConfig::Viewer
    } else {
        PlayerConfig::Local
    };

    let window_width = match config {
        PlayerConfig::TwoLocal => DEFAULT_WINDOW_WIDTH * 2,
        PlayerConfig::TwoRemote => DEFAULT_WINDOW_WIDTH * 2,
        _ => DEFAULT_WINDOW_WIDTH,
    };

    // Create a Glutin window.
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("TETRIS", [window_width, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(OPENGL_VERSION, config);
    let mut fall_speed_divide: u64 = 50;
    let mut freeze: u64 = 50;

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
