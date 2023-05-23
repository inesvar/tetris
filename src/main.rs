extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use app::PlayerConfig;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent, ReleaseEvent};
use clap::Parser;

use tetromino::Tetromino;
use crate::app::App;
use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OPENGL_VERSION};
use crate::assets::Assets;

mod app;
mod assets;
mod block;
mod circular_buffer;
mod keyboard;
mod local_player;
mod point;
mod remote_player;
mod render;
mod rotation;
mod settings;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;
mod translate_rotate;
mod ui;
mod player_screen;

#[derive(Parser, Debug)]
struct Args {
    // two players
    #[arg(short, long)]
    two_players: bool,
    // remote playing
    #[arg(short, long)]
    receive_remote: bool,
}

// TO CHECK OUT THE COMMAND LINE OPTIONS use the following template
// cargo run -- -h

fn main() {
    // Create a Glutin window.
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();
    // Check the command line arguments.
    let args = Args::parse();

    let config: PlayerConfig = match (args.two_players, args.receive_remote) {
        (false, false) => { println!("one player"); PlayerConfig::OneLocal},
        (false, true) => { println!("server role"); PlayerConfig::OneRemote},
        (true, false) => { println!("two local players UNIMPLEMENTED YET"); PlayerConfig::TwoLocal},
        _ => { println!("one local one remote player UNIMPLEMENTED YET"); PlayerConfig::OneLocalOneRemote},
    };
    // Create a new game and run it.
    let mut app = App::new(OPENGL_VERSION, config);
    let mut gravity: u64 = 50;
    let mut freeze: u64 = 50;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args, gravity, freeze);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }

        match app.clock {
            i if i <= 5.0 => {gravity = 50; freeze = 50},
            i if i <= 10.0 => {gravity = 40; freeze = 50},
            i if i <= 15.0 => {gravity = 30; freeze = 50},
            i if i <= 20.0 => {gravity = 20; freeze = 50},
            _ => {gravity = 10; freeze = 50},
        }
    }
}
