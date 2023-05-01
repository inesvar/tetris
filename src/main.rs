extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use tetromino::{Tetromino, NewTetromino};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::{color, Transformed};

mod block;
mod point;
mod settings;
mod assets;

use crate::assets::{Assets, TetrisColor};
mod tetris_grid;
mod tetromino;

use crate::settings::{BG_COLOR, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use tetris_grid::TetrisGrid;

pub struct App<'a> {
    gl: GlGraphics,
    grid: TetrisGrid,
    assets: Assets<'a>,
    clock: f64,
    frame_counter: u64,
    active_tetromino: Tetromino
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            let title_transform = ctx.transform.trans(180.0, 50.0);
            graphics::text::Text::new_color(color::WHITE, 16).draw(
                "T", &mut self.assets.tetris_font, &ctx.draw_state,
                title_transform,
                gl
            ).unwrap();

            let timer_transform = ctx.transform.trans(0.0, 200.0);
            graphics::text::Text::new_color(color::WHITE, 16).draw(
                format!("Elapsed: {}s", self.clock.to_string()).as_str(), &mut self.assets.main_font, &ctx.draw_state,
                timer_transform,
                gl
            ).unwrap();

            self.grid.render(args, &ctx, gl, &self.assets);

            self.active_tetromino.render(self.grid.transform, &ctx, gl, &self.assets);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.clock += args.dt;
        self.frame_counter = self.frame_counter.wrapping_add(1);

        if self.frame_counter % 50 == 0 {
            if let NewTetromino::Error = self.active_tetromino.fall(&self.grid.rows) {
                self.grid.freeze_tetromino(&mut self.active_tetromino);
                self.active_tetromino = Tetromino::new(TetrisColor::random(), &mut [2, 14, 1, 0, -1, 0, 0, 0, 0, 1]);
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create a Glutin window.
    let mut window: piston_window::PistonWindow = WindowSettings::new("TETRIS", [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
        .graphics_api(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let assets = Assets::new(assets_folder);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        assets,
        grid: TetrisGrid::new(10, 22),
        clock: 0.0,
        frame_counter: 0,
        active_tetromino: Tetromino::new(TetrisColor::random(), &mut [2, 20, 1, 0, -1, 0, 0, 0, 0, 1]),
    };
    /*app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 20, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 17, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 14, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 11, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 8, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 5, 1, 0, -1, 0, 0, 0, 0, 1]));
    app.grid.freeze_tetromino(&mut Tetromino::new(TetrisColor::random(), &mut [5, 2, 1, 0, -1, 0, 0, 0, 0, 1]));*/

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
