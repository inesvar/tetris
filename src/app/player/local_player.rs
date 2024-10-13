//! Defines the general implementation of [LocalPlayer].
use super::back_end::{new_tetromino_bag, TetrisGrid, Tetromino, TranslationRotation};
use super::{
    circular_buffer::CircularBuffer, pressed_keys::PressedKeys, LocalPlayer, PlayerScreen,
};
use crate::{app::Countdown, app::PlayerConfig, assets::Assets, once, settings::*};
use graphics::types::Matrix2d;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;

impl LocalPlayer {
    pub fn new(player_config: &PlayerConfig) -> Self {
        let grid = TetrisGrid::new(DEFAULT_GRID_X, DEFAULT_GRID_Y, NB_COLUMNS, NB_ROWS);
        let rng = Pcg32::seed_from_u64(0);
        let bag_of_tetromino = Vec::new();
        let first_tetromino = Tetromino::default();
        let fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        let mut remote_ip = String::from("");
        let mut sender = false;
        if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: ip,
        } = player_config
        {
            sender = true;
            remote_ip = ip.to_string();
        }

        let player_screen = PlayerScreen {
            grid,
            score: 0,
            game_over: false,
            new_completed_lines: 0,
            active_tetromino: first_tetromino,
            saved_tetromino: None,
            fifo_next_tetromino,
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        };

        LocalPlayer {
            player_screen,
            keyboard: PressedKeys::new(),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            bag_of_tetromino,
            sender,
            remote_ip,
            garbage_to_be_added: 0,
            rng,
        }
    }

    pub fn renew(&mut self, seed: u64) {
        self.player_screen.grid.null();
        self.player_screen.score = 0;
        self.player_screen.saved_tetromino = None;
        self.player_screen.ghost_tetromino = None;
        self.rng = Pcg32::seed_from_u64(seed);
        self.bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut self.rng);
        self.player_screen.active_tetromino = Tetromino::new(
            self.bag_of_tetromino.pop().unwrap(),
            &self.player_screen.grid.matrix[..],
        )
        .unwrap();
        self.player_screen.fifo_next_tetromino =
            CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = self.bag_of_tetromino.pop() {
                self.player_screen
                    .fifo_next_tetromino
                    .push(Tetromino::new_unchecked(t));
            } else {
                self.bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut self.rng);
                if let Some(t) = self.bag_of_tetromino.pop() {
                    self.player_screen
                        .fifo_next_tetromino
                        .push(Tetromino::new_unchecked(t));
                } else {
                    unreachable!();
                }
            }
        }
        self.freeze_frame = 0;
        self.player_screen.game_over = false;
    }

    pub fn add_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added = completed_lines;
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        let lines = self.player_screen.new_completed_lines;
        self.player_screen.new_completed_lines = 0;
        lines
    }

    pub fn start(&mut self) {
        self.player_screen.grid.null();
        self.player_screen.game_over = false;
    }

    pub fn get_game_over(&self) -> bool {
        self.player_screen.game_over
    }

    pub fn declare_game_over(&mut self) {
        self.player_screen.game_over = true;
        self.player_screen.saved_tetromino = None;
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

    pub(in crate::app) fn countdown(&mut self, i: &Countdown) {
        match i {
            Countdown::One => self
                .player_screen
                .grid
                .one(self.player_screen.fifo_next_tetromino.get(1).unwrap()),
            Countdown::Two => self
                .player_screen
                .grid
                .two(self.player_screen.fifo_next_tetromino.get(0).unwrap()),
            Countdown::Three => self
                .player_screen
                .grid
                .three(self.player_screen.active_tetromino),
        }
    }
}

impl LocalPlayer {
    /// Sets a new active_tetromino when the precedent one is frozen.
    pub(super) fn get_new_tetromino(&mut self) {
        // Refill the bag if necessary
        if self.bag_of_tetromino.is_empty() {
            self.bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut self.rng);
        }
        // Check if there's enough place on the grid for a new tetromino
        let possible_active = self.player_screen.fifo_next_tetromino.pop().unwrap();
        if possible_active
            .check_possible(&self.player_screen.grid.matrix, TranslationRotation::null())
            .is_err()
        {
            // If not, it's a lock out situation
            // set the game_over flag and return the tetromino to the bag
            self.declare_game_over();
            self.player_screen
                .fifo_next_tetromino
                .push_front(possible_active);
            return;
        }
        // Add a new tetromino to the file to replace the one that was taken
        self.player_screen
            .fifo_next_tetromino
            .push(Tetromino::new_unchecked(
                self.bag_of_tetromino.pop().unwrap(),
            ));
        self.player_screen.active_tetromino = possible_active;
    }

    /// Sends the player screen to the remote player and resets the new_completed_lines attribute.
    pub(in crate::app) fn send_serialized(&mut self) {
        if let Ok(stream) = TcpStream::connect(&self.remote_ip) {
            serde_cbor::to_writer::<TcpStream, PlayerScreen>(stream, &self.player_screen).unwrap();
        }
        once!("sent serialized data to the remote");
        // Set the number of completed lines to 0
        if self.player_screen.new_completed_lines != 0 {
            once!(
                "the {} completed lines were sent to the adversary and they were reset to 0",
                self.player_screen.new_completed_lines
            );
            self.player_screen.new_completed_lines = 0;
        }
    }
}
