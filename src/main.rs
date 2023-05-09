extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::assets::Assets;

use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent, ReleaseEvent};
use tetromino::Tetromino;
use tetromino_kind::TetrominoKind;
use translate_rotate::TranslateRotate;

mod app;
mod assets;
mod block;
mod keyboard;
mod point;
mod rotation;
mod settings;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;
mod translate_rotate;

use crate::app::App;
use crate::settings::{BAG_SIZE, BG_COLOR, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, FALL_KEYS, HARD_DROP_KEYS, HOLD_TETROMINO_KEYS, LEFT_KEYS, OPENGL_VERSION, RESTART_KEYS, RIGHT_KEYS, ROTATE_CLOCKWISE_KEYS, ROTATE_COUNTERCLOCKWISE_KEYS};
use tetris_grid::TetrisGrid;

fn main() {
    // Create a Glutin window.
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(OPENGL_VERSION)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(OPENGL_VERSION);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
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
    }
}
