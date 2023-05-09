use graphics::color;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use crate::keyboard::Keyboard;
use crate::{Assets, BAG_SIZE, BG_COLOR, FALL_KEYS, LEFT_KEYS, RIGHT_KEYS, TetrisGrid, Tetromino, TetrominoKind, TranslateRotate};
use graphics::Transformed;

pub struct App<'a> {
    pub gl: GlGraphics,
    pub grid: TetrisGrid,
    pub assets: Assets<'a>,
    pub clock: f64,
    pub frame_counter: u64,
    pub score: u64,
    pub active_tetromino: Tetromino,
    pub ghost_tetromino: Option<Tetromino>,
    pub saved_tetromino: Option<Tetromino>,
    pub keyboard: Keyboard,
    pub running: bool,
    pub freeze_frame: u64,
    pub bag_of_tetromino: Vec<TetrominoKind>,
}


impl App<'_> {
    pub(crate) fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            if self.running {
                let title_transform = ctx.transform.trans(180.0, 50.0);
                graphics::text::Text::new_color(color::WHITE, 16)
                    .draw(
                        "T",
                        &mut self.assets.tetris_font,
                        &ctx.draw_state,
                        title_transform,
                        gl,
                    )
                    .unwrap();
            } else {
                let restart_transform = ctx.transform.trans(180.0, 50.0);
                graphics::text::Text::new_color(color::WHITE, 16)
                    .draw(
                        "Press R to restart",
                        &mut self.assets.main_font,
                        &ctx.draw_state,
                        restart_transform,
                        gl,
                    )
                    .unwrap();
            }

            let timer_transform = ctx.transform.trans(0.0, 200.0);
            graphics::text::Text::new_color(color::WHITE, 16)
                .draw(
                    format!("Elapsed: {:.2}s", self.clock).as_str(),
                    &mut self.assets.main_font,
                    &ctx.draw_state,
                    timer_transform,
                    gl,
                )
                .unwrap();

            let score_transform = ctx.transform.trans(0.0, 250.0);
            graphics::text::Text::new_color(color::WHITE, 16)
                .draw(
                    format!("Score: {}", self.score).as_str(),
                    &mut self.assets.main_font,
                    &ctx.draw_state,
                    score_transform,
                    gl,
                )
                .unwrap();

            self.grid.render(args, &ctx, gl, &self.assets);

            if let Some(ghost) = self.ghost_tetromino {
                ghost.render(self.grid.transform, &ctx, gl, &self.assets);
            }

            self.active_tetromino
                .render(self.grid.transform, &ctx, gl, &self.assets);

            if let Some(saved) = self.saved_tetromino {
                let transform = ctx.transform.trans(-70.0, 50.0);
                saved.render(transform, &ctx, gl, &self.assets);
            }
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        self.keyboard.update();

        if !self.running {
            return;
        }

        self.clock += args.dt;
        self.frame_counter = self.frame_counter.wrapping_add(1);

        self.ghost_tetromino = Some(self.active_tetromino.make_ghost_copy());
        if let Some(ghost) = self.ghost_tetromino.as_mut() {
            ghost.hard_drop(&self.grid.rows);
        }

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if self.frame_counter == self.freeze_frame
            && self
            .active_tetromino
            .check_possible(&self.grid.rows, TranslateRotate::fall())
            .is_err()
        {
            self.grid.freeze_tetromino(&mut self.active_tetromino);
            self.get_new_tetromino();
        }

        // move the tetromino down to emulate its fall
        if self.frame_counter % 50 == 0 {
            if self.active_tetromino.fall(&self.grid.rows).is_err() {
                self.freeze_frame = self.frame_counter + 50;
            }
        }

        // Translate the tetromino on long key press
        if self.frame_counter % 10 == 0 {
            if self.keyboard.is_any_pressed(&FALL_KEYS) {
                if self.active_tetromino.fall(&self.grid.rows).is_err() {
                    self.freeze_frame = self.frame_counter + 50;
                }
            } else if self.keyboard.is_any_delay_pressed(&LEFT_KEYS) {
                self.active_tetromino.left(&self.grid.rows);
            } else if self.keyboard.is_any_delay_pressed(&RIGHT_KEYS) {
                self.active_tetromino.right(&self.grid.rows);
            }
        }

        self.grid.update();

        self.score += self.grid.nb_lines_cleared_last_frame as u64;
    }

    fn game_over(&mut self) {
        println!("game over");
        self.grid.null();
        self.running = false;
    }

    pub(crate) fn get_new_tetromino(&mut self) {
        if self.bag_of_tetromino.is_empty() {
            self.bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
        }
        match Tetromino::new(self.bag_of_tetromino.pop().unwrap(), &self.grid.rows) {
            Some(t) => {
                self.active_tetromino = t;
            }
            None => self.game_over(),
        };
    }
}