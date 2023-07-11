use graphics::types::Matrix2d;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;

use crate::assets::Assets;
use crate::circular_buffer::CircularBuffer;
use crate::player_screen::PlayerScreen;
use crate::tetromino_bag::new_random_bag;
use crate::translate_rotate::TranslateRotate;
use crate::{
    keyboard::Keyboard, tetris_grid::TetrisGrid, tetromino::Tetromino,
    tetromino_kind::TetrominoKind,
};
use crate::{once, settings::*};
use opengl_graphics::GlGraphics;
use piston::Key;
use piston_window::Context;

use crate::ui::button::Button;
use crate::ui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalPlayer {
    player_screen: PlayerScreen,
    keyboard: Keyboard,
    freeze_frame: u64,
    bag_of_tetromino: Vec<TetrominoKind>,
    game_over: bool,
    sender: bool,
    garbage_to_be_added: u64,
    #[serde(skip, default = "new_pcg")]
    rng: Pcg32,
}

fn new_pcg() -> Pcg32 {
    Pcg32::seed_from_u64(0)
}

impl LocalPlayer {
    pub fn new(seed: u64, sender: bool) -> Self {
        let grid = TetrisGrid::new(DEFAULT_GRID_X, DEFAULT_GRID_Y, NB_COLUMNS, NB_ROWS);
        let mut rng = Pcg32::seed_from_u64(seed);
        let mut bag_of_tetromino = new_random_bag(BAG_SIZE, &mut rng);
        let first_tetromino =
            Tetromino::new_collision(bag_of_tetromino.pop().unwrap(), &grid.rows[..]).unwrap();
        let mut fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = bag_of_tetromino.pop() {
                fifo_next_tetromino.push(Tetromino::new(t));
            } else {
                bag_of_tetromino = new_random_bag(BAG_SIZE, &mut rng);
                if let Some(t) = bag_of_tetromino.pop() {
                    fifo_next_tetromino.push(Tetromino::new(t));
                } else {
                    unreachable!();
                }
            }
        }

        let player_screen = PlayerScreen {
            grid,
            score: 0,
            new_completed_lines: 0,
            active_tetromino: first_tetromino,
            saved_tetromino: None,
            fifo_next_tetromino,
            ghost_tetromino: None,
        };

        LocalPlayer {
            player_screen,
            keyboard: Keyboard::new(),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            bag_of_tetromino,
            game_over: false,
            sender,
            garbage_to_be_added: 0,
            rng,
        }
    }

    fn get_new_tetromino(&mut self) {
        if self.bag_of_tetromino.is_empty() {
            self.bag_of_tetromino = new_random_bag(BAG_SIZE, &mut self.rng);
        }
        let possible_active = self.player_screen.fifo_next_tetromino.pop().unwrap();
        self.player_screen
            .fifo_next_tetromino
            .push(Tetromino::new(self.bag_of_tetromino.pop().unwrap()));
        if possible_active
            .check_possible(&self.player_screen.grid.rows, TranslateRotate::null())
            .is_err()
        {
            self.game_over = true;
            return;
        }
        self.player_screen.active_tetromino = possible_active;
    }

    pub fn add_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added = completed_lines;
    }

    pub fn send_serialized(&self) {
        if let Ok(stream) = TcpStream::connect(VIEWER_IP) {
            serde_cbor::to_writer::<TcpStream, PlayerScreen>(stream, &self.player_screen).unwrap();
        }
        once!("sent serialized data to {}", VIEWER_IP);
    }
}

impl LocalPlayer {
    pub fn restart(&mut self) {
        self.game_over = false;
        self.player_screen.score = 0;
        self.keyboard = Keyboard::new();
        self.player_screen.grid.null();
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn render(
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        self.player_screen.render(transform, ctx, gl, assets);
    }

    pub fn update(&mut self, frame_counter: u64, gravity: u64, freeze: u64) {
        self.keyboard.update();

        let mut ghost = self.player_screen.active_tetromino.make_ghost_copy();
        ghost.hard_drop(&self.player_screen.grid.rows);
        self.player_screen.ghost_tetromino = Some(ghost);

        // Add the garbage
        if self.garbage_to_be_added != 0
            && self
                .player_screen
                .grid
                .add_garbage(self.garbage_to_be_added)
                .is_err()
        {
            self.game_over = true;
        }
        self.garbage_to_be_added = 0;

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame
            && self
                .player_screen
                .active_tetromino
                .check_possible(&self.player_screen.grid.rows, TranslateRotate::fall())
                .is_err()
        {
            self.player_screen.new_completed_lines = self
                .player_screen
                .grid
                .freeze_tetromino(&mut self.player_screen.active_tetromino);
            if self.player_screen.new_completed_lines != 0 {
                println!(
                    "{} lines were completed",
                    self.player_screen.new_completed_lines
                );
            }
            self.player_screen.score += self.player_screen.new_completed_lines;
            self.get_new_tetromino();
        }

        // move the tetromino down to emulate its fall
        if frame_counter % gravity == 0
            && self
                .player_screen
                .active_tetromino
                .fall(&self.player_screen.grid.rows)
                .is_err()
            && self.freeze_frame < frame_counter
        {
            self.freeze_frame = frame_counter + freeze;
        }

        // Translate the tetromino on long key press
        if frame_counter % 5 == 0 {
            if self.keyboard.is_any_pressed(&FALL_KEYS) {
                if self
                    .player_screen
                    .active_tetromino
                    .fall(&self.player_screen.grid.rows)
                    .is_err()
                    && self.freeze_frame < frame_counter
                {
                    self.freeze_frame = frame_counter + freeze;
                }
            } else if self.keyboard.is_any_delay_pressed(&LEFT_KEYS) {
                self.player_screen
                    .active_tetromino
                    .left(&self.player_screen.grid.rows);
            } else if self.keyboard.is_any_delay_pressed(&RIGHT_KEYS) {
                self.player_screen
                    .active_tetromino
                    .right(&self.player_screen.grid.rows);
            }
        }
        // Send the player_screen data if necessary
        if self.sender {
            self.send_serialized();
        }

        // Set the number of completed lines to 0
        if self.player_screen.new_completed_lines != 0 {
            println!(
                "the {} completed lines were sent to the adversary and they were reset to 0",
                self.player_screen.new_completed_lines
            );
            self.player_screen.new_completed_lines = 0;
        }
    }

    pub fn handle_key_press(&mut self, key: Key, running: bool) -> KeyPress {
        self.keyboard.set_pressed(key);

        if !running && !self.keyboard.is_any_pressed(&RESTART_KEYS) {
            return KeyPress::Other;
        }

        // Pressed once events
        if self.keyboard.is_any_pressed(&ROTATE_CLOCKWISE_KEYS) {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_clockwise(&self.player_screen.grid.rows);
        } else if self.keyboard.is_any_pressed(&ROTATE_COUNTERCLOCKWISE_KEYS) {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_counterclockwise(&self.player_screen.grid.rows);
        }

        if self.keyboard.is_any_pressed(&HARD_DROP_KEYS) {
            // hard drop the tetromino
            self.player_screen
                .active_tetromino
                .hard_drop(&self.player_screen.grid.rows);
            self.player_screen.new_completed_lines = self
                .player_screen
                .grid
                .freeze_tetromino(&mut self.player_screen.active_tetromino);
            if self.player_screen.new_completed_lines != 0 {
                println!(
                    "{} lines were completed",
                    self.player_screen.new_completed_lines
                );
            }
            self.player_screen.score += self.player_screen.new_completed_lines;
            self.get_new_tetromino();
        }

        if self.keyboard.is_any_pressed(&HOLD_TETROMINO_KEYS) {
            // hold the tetromino
            if let Some(mut saved) = self.player_screen.saved_tetromino {
                self.player_screen.active_tetromino.reset_position();

                std::mem::swap(&mut saved, &mut self.player_screen.active_tetromino);
                self.player_screen.saved_tetromino = Some(saved);
            } else {
                self.player_screen.active_tetromino.reset_position();

                self.player_screen.saved_tetromino = Some(self.player_screen.active_tetromino);
                self.get_new_tetromino();
            }
        }

        if self.keyboard.is_any_pressed(&LEFT_KEYS) {
            self.player_screen
                .active_tetromino
                .left(&self.player_screen.grid.rows);
        } else if self.keyboard.is_any_pressed(&RIGHT_KEYS) {
            self.player_screen
                .active_tetromino
                .right(&self.player_screen.grid.rows);
        }

        if self.keyboard.is_any_pressed(&RESTART_KEYS) {
            KeyPress::Restart
        } else {
            KeyPress::Other
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}

pub enum KeyPress {
    Restart,
    Other,
}
