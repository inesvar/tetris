use super::{
    circular_buffer::CircularBuffer, pressed_keys::PressedKeys, LocalPlayer, PlayerScreen,
};
use crate::assets::Assets;
use crate::back_end::{new_tetromino_bag, TetrisGrid, Tetromino, TranslationRotation};
use crate::{once, settings::*};
use graphics::types::Matrix2d;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;

impl LocalPlayer {
    pub fn new(seed: u64, sender: bool) -> Self {
        let grid = TetrisGrid::new(DEFAULT_GRID_X, DEFAULT_GRID_Y, NB_COLUMNS, NB_ROWS);
        let mut rng = Pcg32::seed_from_u64(seed);
        let mut bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut rng);
        let first_tetromino =
            Tetromino::new(bag_of_tetromino.pop().unwrap(), &grid.matrix[..]).unwrap();
        let mut fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = bag_of_tetromino.pop() {
                fifo_next_tetromino.push(Tetromino::new_unchecked(t));
            } else {
                bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut rng);
                if let Some(t) = bag_of_tetromino.pop() {
                    fifo_next_tetromino.push(Tetromino::new_unchecked(t));
                } else {
                    unreachable!();
                }
            }
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
            garbage_to_be_added: 0,
            rng,
        }
    }

    pub fn add_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added = completed_lines;
    }

    pub fn restart(&mut self) {
        self.player_screen.game_over = false;
        self.player_screen.score = 0;
        self.player_screen.grid.null();
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
    pub(super) fn send_serialized(&mut self) {
        if let Ok(stream) = TcpStream::connect(VIEWER_IP) {
            serde_cbor::to_writer::<TcpStream, PlayerScreen>(stream, &self.player_screen).unwrap();
        }
        once!("sent serialized data to {}", VIEWER_IP);
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
