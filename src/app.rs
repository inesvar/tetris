use crate::keyboard::Keyboard;
use crate::{
    Assets, TetrisGrid, Tetromino, TetrominoKind, TranslateRotate};
use crate::settings::*;
use graphics::color;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};
use piston_window::Key;
use circular_buffer::CircularBuffer;

pub struct App<'a> {
    gl: GlGraphics,
    grid: TetrisGrid,
    assets: Assets<'a>,
    clock: f64,
    frame_counter: u64,
    score: u64,
    active_tetromino: Tetromino,
    ghost_tetromino: Tetromino,
    saved_tetromino: Option<Tetromino>,
    keyboard: Keyboard,
    running: bool,
    freeze_frame: u64,
    bag_of_tetromino: Vec<TetrominoKind>,
    fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>, // push_back / pop_front
}

impl App<'_> {
    pub fn new(gl_version: OpenGL) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let grid = TetrisGrid::new(10, 22);

        let mut bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
        let first_tetromino = Tetromino::new_collision(bag_of_tetromino.pop().unwrap(), &grid.rows).unwrap();
        let mut fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = bag_of_tetromino.pop() {
                fifo_next_tetromino.push_back(Tetromino::new(t));
            } else {
                bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
                if let Some(t) = bag_of_tetromino.pop() {
                    fifo_next_tetromino.push_back(Tetromino::new(t));
                } else {
                    unreachable!();
                }
            }
        }

        App {
            gl: GlGraphics::new(gl_version),
            assets: Assets::new(assets_folder),
            grid,
            bag_of_tetromino,
            active_tetromino: first_tetromino,
            ghost_tetromino: first_tetromino.clone(),
            clock: 0.0,
            frame_counter: 0,
            score: 0,
            saved_tetromino: None,
            running: true,
            keyboard: Keyboard::new(),
            freeze_frame: u64::MAX, // that's about 10 billion years at 60fps
            fifo_next_tetromino,
        }
    }
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

            self.ghost_tetromino.render(self.grid.transform, &ctx, gl, &self.assets);

            self.active_tetromino
                .render(self.grid.transform, &ctx, gl, &self.assets);

            if let Some(saved) = self.saved_tetromino {
                let transform = ctx.transform.trans(-70.0, 50.0);
                saved.render(transform, &ctx, gl, &self.assets);
            }

            for i in 0..NB_NEXT_TETROMINO {
                let transform = ctx.transform.trans(BLOCK_SIZE * 16.0, 5.0 * BLOCK_SIZE + 4.0 * BLOCK_SIZE * i as f64);
                self.fifo_next_tetromino.get(i).unwrap().render(transform, &ctx, gl, &self.assets);
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

        self.ghost_tetromino = self.active_tetromino.make_ghost_copy();
        self.ghost_tetromino.hard_drop(&self.grid.rows);

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
        if self.frame_counter % 5 == 0 {
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
        self.active_tetromino = self.fifo_next_tetromino.pop_front().unwrap();
        if self.active_tetromino.check_possible(&self.grid.rows, TranslateRotate::null()).is_err() {
            self.game_over()
        }
        self.fifo_next_tetromino.push_back(Tetromino::new(self.bag_of_tetromino.pop().unwrap()));
    }

    pub fn handle_key_press(&mut self, key: Key) {
        self.keyboard.set_pressed(key);

        if self.keyboard.is_any_pressed(&RESTART_KEYS) {
            self.running = true;
        }

        if !self.running {
            return;
        }

        // Pressed once events
        if self.keyboard.is_any_pressed(&ROTATE_CLOCKWISE_KEYS) {
            // rotate once the tetromino
            self.active_tetromino.turn_clockwise(&self.grid.rows);
        } else if self.keyboard.is_any_pressed(&ROTATE_COUNTERCLOCKWISE_KEYS) {
            // rotate once the tetromino
            self.active_tetromino.turn_counterclockwise(&self.grid.rows);
        }

        if self.keyboard.is_any_pressed(&HARD_DROP_KEYS) {
            // hard drop the tetromino
            self.active_tetromino.hard_drop(&self.grid.rows);
            self.grid.freeze_tetromino(&mut self.active_tetromino);
            self.get_new_tetromino();
        }

        if self.keyboard.is_any_pressed(&HOLD_TETROMINO_KEYS) {
            // hold the tetromino
            if let Some(mut saved) = self.saved_tetromino {
                self.active_tetromino.reset_position();

                std::mem::swap(&mut saved, &mut self.active_tetromino);
                self.saved_tetromino = Some(saved);
            } else {
                self.active_tetromino.reset_position();

                self.saved_tetromino = Some(self.active_tetromino);
                self.get_new_tetromino();
            }
        }

        if self.keyboard.is_any_pressed(&LEFT_KEYS) {
            self.active_tetromino.left(&self.grid.rows);
        } else if self.keyboard.is_any_pressed(&RIGHT_KEYS) {
            self.active_tetromino.right(&self.grid.rows);
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}
