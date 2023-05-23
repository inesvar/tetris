use std::net::TcpStream;

use crate::assets::Assets;
use crate::settings::*;
use crate::translate_rotate::TranslateRotate;
use crate::{
    keyboard::Keyboard, tetris_grid::TetrisGrid, tetromino::Tetromino,
    tetromino_kind::TetrominoKind,
};
use crate::circular_buffer::CircularBuffer;
use graphics::{color, Transformed};
use opengl_graphics::GlGraphics;
use piston::Key;
use piston_window::Context;
use piston_window::RenderArgs;
use serde::{Serialize, Deserialize};
use crate::ui::text::Text;

#[derive(Serialize, Deserialize)]
pub struct LocalPlayer {
    grid: TetrisGrid,
    score: u64,
    active_tetromino: Tetromino,
    ghost_tetromino: Tetromino,
    saved_tetromino: Option<Tetromino>,
    keyboard: Keyboard,
    freeze_frame: u64,
    bag_of_tetromino: Vec<TetrominoKind>,
    fifo_next_tetromino: CircularBuffer<NB_NEXT_TETROMINO, Tetromino>,
    game_over: bool,
}

pub trait Player {
    fn render(&mut self, ctx: Context, gl: &mut GlGraphics, args: &RenderArgs, assets: &mut Assets);
    fn update(&mut self, frame_counter: u64);
    fn handle_key_press(&mut self, key: Key, running: bool) -> KeyPress;
    fn handle_key_release(&mut self, key: Key);
    fn restart(&mut self);
    fn game_over(&self) -> bool;
}

impl LocalPlayer {
    pub fn new() -> Self {
        let grid = TetrisGrid::new(150.0, 70.0, NB_COLUMNS, NB_ROWS);

        let mut bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
        let first_tetromino =
            Tetromino::new_collision(bag_of_tetromino.pop().unwrap(), &grid.rows[..]).unwrap();
        let mut fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = bag_of_tetromino.pop() {
                fifo_next_tetromino.push(Tetromino::new(t));
            } else {
                bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
                if let Some(t) = bag_of_tetromino.pop() {
                    fifo_next_tetromino.push(Tetromino::new(t));
                } else {
                    unreachable!();
                }
            }
        }

        LocalPlayer {
            grid,
            score: 0,
            active_tetromino: first_tetromino,
            ghost_tetromino: first_tetromino,
            saved_tetromino: None,
            keyboard: Keyboard::new(),
            freeze_frame: u64::MAX, // that's about 10 billion years at 60fps
            bag_of_tetromino,
            fifo_next_tetromino,
            game_over: false,
        }
    }

    fn get_new_tetromino(&mut self) {
        if self.bag_of_tetromino.is_empty() {
            self.bag_of_tetromino = TetrominoKind::new_random_bag(BAG_SIZE);
        }
        let possible_active = self.fifo_next_tetromino.pop().unwrap();
        self.fifo_next_tetromino
            .push(Tetromino::new(self.bag_of_tetromino.pop().unwrap()));
        if possible_active
            .check_possible(&self.grid.rows, TranslateRotate::null())
            .is_err()
        {
            self.game_over = true;
            return;
        }
        self.active_tetromino = possible_active;
    }

    pub fn send_serialized(&self) {
        println!("trying to connect...");
        if let Ok(stream) = TcpStream::connect("172.16.2.48:16000") {
            println!("connected");
            serde_cbor::to_writer::<TcpStream, LocalPlayer>(stream, self).unwrap();
            println!("sent");
        }
    }
}

impl Player for LocalPlayer {
    fn restart(&mut self) {
        self.game_over = false;
        self.score = 0;
        self.keyboard = Keyboard::new();
        self.grid.null();
    }

    fn game_over(&self) -> bool {
        self.game_over
    }

    fn render(
        &mut self,
        ctx: Context,
        gl: &mut GlGraphics,
        args: &RenderArgs,
        assets: &mut Assets,
    ) {
        let score_text = Text::new(format!("Score: {}", self.score), 16, 0.0, 250.0, color::WHITE);
        score_text.render(ctx.transform, &ctx, gl, &mut assets.main_font);

        self.grid.render(ctx.transform, &ctx, gl, assets);

        self.ghost_tetromino.render(self.grid.transform, &ctx, gl, assets);

        self.active_tetromino.render(self.grid.transform, &ctx, gl, assets);

        if let Some(saved) = self.saved_tetromino {
            let transform = self.grid.transform.trans(-100.0 - (saved.center.x as f64 * BLOCK_SIZE), 50.0);
            saved.render(transform, &ctx, gl, assets);
        }

        for i in 0..NB_NEXT_TETROMINO {
            let transform = self.grid.transform.trans(
                self.grid.total_width,
                4.0 * BLOCK_SIZE * (i as f64 + 0.5),
            );
            self.fifo_next_tetromino
                .get(i)
                .unwrap()
                .render(transform, &ctx, gl, assets);
        }
    }

    fn update(&mut self, frame_counter: u64) {
        self.keyboard.update();

        self.ghost_tetromino = self.active_tetromino.make_ghost_copy();
        self.ghost_tetromino.hard_drop(&self.grid.rows);

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame
            && self
            .active_tetromino
            .check_possible(&self.grid.rows, TranslateRotate::fall())
            .is_err()
        {
            self.score += self.grid.freeze_tetromino(&mut self.active_tetromino);
            self.get_new_tetromino();
        }

        // move the tetromino down to emulate its fall
        if frame_counter % 50 == 0 && self.active_tetromino.fall(&self.grid.rows).is_err() {
            self.freeze_frame = frame_counter + 50;
        }

        // Translate the tetromino on long key press
        if frame_counter % 5 == 0 {
            if self.keyboard.is_any_pressed(&FALL_KEYS) {
                if self.active_tetromino.fall(&self.grid.rows).is_err() {
                    self.freeze_frame = frame_counter + 50;
                }
            } else if self.keyboard.is_any_delay_pressed(&LEFT_KEYS) {
                self.active_tetromino.left(&self.grid.rows);
            } else if self.keyboard.is_any_delay_pressed(&RIGHT_KEYS) {
                self.active_tetromino.right(&self.grid.rows);
            }
        }
        println!("got there at least");

        self.send_serialized();
    }

    fn handle_key_press(&mut self, key: Key, running: bool) -> KeyPress {
        self.keyboard.set_pressed(key);

        if !running && !self.keyboard.is_any_pressed(&RESTART_KEYS) {
            return KeyPress::Other;
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
            self.score += self.grid.freeze_tetromino(&mut self.active_tetromino);
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

        if self.keyboard.is_any_pressed(&RESTART_KEYS) {
            KeyPress::Restart
        } else {
            KeyPress::Other
        }
    }

    fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}

pub enum KeyPress {
    Restart,
    Other,
}
