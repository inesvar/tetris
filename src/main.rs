extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::assets::Assets;
use graphics::{color, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent, ReleaseEvent};
use tetromino::Tetromino;
use tetromino_kind::TetrominoKind;
use translate_rotate::TranslateRotate;

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
mod app;

use crate::settings::{
    BG_COLOR, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, FALL_KEYS, HARD_DROP_KEYS,
    HOLD_TETROMINO_KEYS, LEFT_KEYS, RESTART_KEYS, RIGHT_KEYS, ROTATE_CLOCKWISE_KEYS,
    ROTATE_COUNTERCLOCKWISE_KEYS, BAG_SIZE,
};
use tetris_grid::TetrisGrid;
use crate::app::App;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create a Glutin window.
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .graphics_api(opengl)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let assets = Assets::new(assets_folder);

    let grid = TetrisGrid::new(10, 22);
    let mut bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        assets,
        active_tetromino: Tetromino::new(bag_of_tetromino.pop().unwrap(), &grid.rows).unwrap(),
        grid,
        clock: 0.0,
        frame_counter: 0,
        running: true,
        score: 0,
        ghost_tetromino: None,
        saved_tetromino: None,
        keyboard: keyboard::Keyboard::new(),
        freeze_frame: u64::MAX,
        bag_of_tetromino,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.keyboard.set_pressed(key);

            if app.keyboard.is_any_pressed(&RESTART_KEYS) {
                app.running = true;
            }

            if !app.running {
                continue;
            }

            // Pressed once events
            if app.keyboard.is_any_pressed(&ROTATE_CLOCKWISE_KEYS) {
                // rotate once the tetromino
                app.active_tetromino.turn_clockwise(&app.grid.rows);
            } else if app.keyboard.is_any_pressed(&ROTATE_COUNTERCLOCKWISE_KEYS) {
                // rotate once the tetromino
                app.active_tetromino.turn_counterclockwise(&app.grid.rows);
            }

            if app.keyboard.is_any_pressed(&HARD_DROP_KEYS) {
                // hard drop the tetromino
                app.active_tetromino.hard_drop(&app.grid.rows);
                app.grid.freeze_tetromino(&mut app.active_tetromino);
                app.get_new_tetromino();
            }

            if app.keyboard.is_any_pressed(&HOLD_TETROMINO_KEYS) {
                // hold the tetromino
                if let Some(mut saved) = app.saved_tetromino {
                    app.active_tetromino.reset_position();

                    std::mem::swap(&mut saved, &mut app.active_tetromino);
                    app.saved_tetromino = Some(saved);
                } else {
                    app.active_tetromino.reset_position();

                    app.saved_tetromino = Some(app.active_tetromino);
                    app.get_new_tetromino();
                }
            }

            if app.keyboard.is_any_pressed(&LEFT_KEYS) {
                app.active_tetromino.left(&app.grid.rows);
            } else if app.keyboard.is_any_pressed(&RIGHT_KEYS) {
                app.active_tetromino.right(&app.grid.rows);
            }
        };
        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.keyboard.set_released(key);
        };
    }
}
