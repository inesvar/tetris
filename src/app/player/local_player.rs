//! Define the general implementation of [LocalPlayer].
use super::core::{GameOverError, TetrisGrid, Tetromino, TetrominoBag};
use super::{
    circular_buffer::CircularBuffer, pressed_keys::PressedKeys, LocalPlayer, PlayerScreen,
};
use crate::{app::Countdown, app::PlayerConfig, once, settings::*};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;

impl LocalPlayer {
    pub fn new(player_config: &PlayerConfig) -> Self {
        let grid = TetrisGrid::default();
        let rng = Pcg32::seed_from_u64(0);
        let bag_of_tetromino = TetrominoBag::default();
        let first_tetromino = Tetromino::default();
        let fifo_next_tetromino = CircularBuffer::new([Tetromino::default(); NB_NEXT_TETROMINO]);
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
        self.player_screen.grid.reset();
        self.player_screen.score = 0;
        self.player_screen.saved_tetromino = None;
        self.player_screen.ghost_tetromino = None;
        self.rng = Pcg32::seed_from_u64(seed);
        self.bag_of_tetromino = TetrominoBag::default();
        self.player_screen.active_tetromino = self.bag_of_tetromino.get(&mut self.rng).into();
        self.player_screen.fifo_next_tetromino = CircularBuffer::new(
            self.bag_of_tetromino
                .get_chunk::<NB_NEXT_TETROMINO>(&mut self.rng)
                .map(|kind| kind.into()),
        );
        self.freeze_frame = 0;
        self.player_screen.game_over = false;
    }

    pub fn add_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added += completed_lines;
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        let lines = self.player_screen.new_completed_lines;
        self.player_screen.new_completed_lines = 0;
        lines
    }

    pub fn start(&mut self) {
        self.player_screen.grid.reset();
        self.player_screen.game_over = false;
    }

    pub fn get_game_over(&self) -> bool {
        self.player_screen.game_over
    }

    pub fn declare_game_over(&mut self) {
        self.player_screen.game_over = true;
        self.player_screen.saved_tetromino = None;
    }

    pub(in crate::app) fn countdown(&mut self, i: &Countdown) {
        match i {
            Countdown::One => self.player_screen.grid.one(
                self.player_screen
                    .fifo_next_tetromino
                    .get(1)
                    .unwrap()
                    .color(),
            ),
            Countdown::Two => self.player_screen.grid.two(
                self.player_screen
                    .fifo_next_tetromino
                    .get(0)
                    .unwrap()
                    .color(),
            ),
            Countdown::Three => self
                .player_screen
                .grid
                .three(self.player_screen.active_tetromino.color()),
        }
    }
}

impl LocalPlayer {
    /// Sets a new active_tetromino when the precedent one is frozen.
    pub(super) fn get_new_tetromino(&mut self) {
        // Check if there's enough place on the grid for a new tetromino
        // TODO this should be done using the grid method and probably all other calls
        // using null()...
        let mut swap = self.bag_of_tetromino.get(&mut self.rng).into();
        self.player_screen
            .fifo_next_tetromino
            .get_front_push_back(&mut swap);
        if swap.can_enter_grid(&self.player_screen.grid) == Err(GameOverError::BlockOut)
        {
            // Set the game_over flag and return the tetromino to the bag.
            self.declare_game_over();
            self.player_screen
                .fifo_next_tetromino
                .get_back_push_front(&mut swap);
            return;
        }
        self.player_screen.active_tetromino = swap;
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

    pub(super) fn lock_down_tetromino(&mut self) -> Result<(), ()> {
        println!("Locking down the active tetromino");
        let res = self
            .player_screen
            .active_tetromino
            .lock_down(&mut self.player_screen.grid);
        match res {
            // if lines were clearing by freezing the tetromino, set the attribute new_completed_lines
            Ok(completed_lines) => {
                self.player_screen.new_completed_lines = completed_lines;
                if self.player_screen.new_completed_lines != 0 {
                    println!(
                        "{} lines were completed",
                        self.player_screen.new_completed_lines
                    );
                }
                self.player_screen.score += self.player_screen.new_completed_lines;
                self.get_new_tetromino();
            }
            // if the tetromino froze above the visible grid, it's game over !
            Err(GameOverError::LockOut) => {
                self.declare_game_over();
                return Err(());
            }
            _ => unreachable!(),
        }

        // Adds garbage to the grid
        match self
            .player_screen
            .grid
            .add_garbage(self.garbage_to_be_added)
        {
            Ok(()) => self.garbage_to_be_added = 0,
            Err(GameOverError::TopOut) => {
                self.declare_game_over();
                return Err(());
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
