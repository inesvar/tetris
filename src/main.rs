extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod point;
mod tetromino;
mod renderer;
mod tetris_grid;
mod block;
mod settings;
mod assets;

use tetris_grid::TetrisGrid;
use crate::assets::Assets;
use crate::settings::{WINDOW_HEIGHT, WINDOW_WIDTH};

use piston_window::*;

pub struct App {
    gl: GlGraphics,
    grid: TetrisGrid,
    assets: Assets
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let (window_width, window_height) = (args.window_size[0], args.window_size[1]);

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(BG_COLOR, gl);

            self.grid.render(&ctx, gl, &self.assets);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // nothing here for now
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create a Glutin window.
    let mut window: PistonWindow = WindowSettings::new("TETRIS", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let assets = Assets::new(assets_folder);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: TetrisGrid::new_random(10, 20, 0.5),
        assets
    };

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
