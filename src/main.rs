extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use tetromino::Tetromino;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod block;
mod point;
mod settings;
mod assets;

use crate::assets::{Assets, TetrisColor};
mod tetris_grid;
mod tetromino;

use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use tetris_grid::TetrisGrid;

use piston_window::*;

pub struct App {
    gl: GlGraphics,
    grid: TetrisGrid,
    assets: Assets,
    counter: u8,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let (window_width, window_height) = (args.window_size[0], args.window_size[1]);

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(BG_COLOR, gl);

            self.grid.render(args, &ctx, gl, &self.assets);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.counter = if self.counter == 255 {
            0
        } else {
            self.counter + 1
        };
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create a Glutin window.
    let mut window: PistonWindow = WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let assets = Assets::new(assets_folder);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        assets,
        grid: TetrisGrid::new(10, 20),
        counter: 100,
    };

    app.grid.add_tetromino(Tetromino::new(TetrisColor::PURPLE, &mut [2, 2, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.add_tetromino(Tetromino::new(TetrisColor::RED, &mut [5, 5, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.add_tetromino(Tetromino::new(TetrisColor::BLUE, &mut [2, 5, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.add_tetromino(Tetromino::new(TetrisColor::GREEN, &mut [5, 2, 1, 0, -1, 0, 0, 0, 0, 1]));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
