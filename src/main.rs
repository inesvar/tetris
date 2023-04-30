extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

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

use tetris_grid::TetrisGrid;

pub struct App {
    gl: GlGraphics,
    grid: TetrisGrid,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let (window_width, window_height) = (args.window_size[0], args.window_size[1]);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG_COLOR, gl);

            self.grid.render(&c, gl);
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
    let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: TetrisGrid::new_random(10, 20, 0.5),
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
