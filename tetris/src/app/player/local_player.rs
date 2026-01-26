//! Define the general implementation of [LocalPlayer].
use super::{pressed_keys::PressedKeys, LocalPlayer, PlayerScreen};
use crate::{
    app::{Countdown, PlayerConfig},
    once,
};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;
use tetris_core::{GameOverError, Tetromino, TetrominoGenerator};

impl LocalPlayer {
    pub fn new(player_config: &PlayerConfig) -> Self {
        let mut rng = Pcg32::seed_from_u64(0);
        let mut tetromino_bag = TetrominoGenerator::default();

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

        let player_screen = PlayerScreen::new(&mut rng, &mut tetromino_bag);

        LocalPlayer {
            player_screen,
            keyboard: PressedKeys::new(),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            tetromino_bag,
            sender,
            remote_ip,
            garbage_to_be_added: 0,
            rng,
        }
    }

    pub fn reset(&mut self, seed: u64) {
        self.rng = Pcg32::seed_from_u64(seed);
        self.tetromino_bag = TetrominoGenerator::default();
        self.player_screen = PlayerScreen::new(&mut self.rng, &mut self.tetromino_bag);
        self.freeze_frame = 0;
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
    /// Replace the active tetromino by a tetromino from the next queue and return
    /// the previously active tetromino.
    fn replace_active_tetromino(&mut self) -> Tetromino {
        let mut swap = self.tetromino_bag.get(&mut self.rng);
        self.player_screen
            .fifo_next_tetromino
            .get_front_push_back(&mut swap);
        std::mem::swap(&mut self.player_screen.active_tetromino, &mut swap);

        swap
    }

    /// Replace the active tetromino by the saved tetromino if it exists (or by a tetromino
    /// from the next queue) and return the previously active tetromino.
    fn replace_active_tetromino_using_stash(&mut self) -> Tetromino {
        if let Some(mut swap) = self.player_screen.saved_tetromino.take() {
            std::mem::swap(&mut swap, &mut self.player_screen.active_tetromino);
            swap
        } else {
            self.replace_active_tetromino()
        }
    }

    pub(super) fn stash(&mut self) -> Result<(), GameOverError> {
        let mut previously_active = self.replace_active_tetromino_using_stash();
        previously_active.reset();
        self.player_screen.saved_tetromino = Some(previously_active);
        self.player_screen
            .active_tetromino
            .try_enter_grid(&self.player_screen.grid)
    }

    fn record_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.player_screen.new_completed_lines += new_completed_lines;
        self.player_screen.score += new_completed_lines;
    }

    pub(super) fn lock_down(&mut self) -> Result<(), GameOverError> {
        let previously_active = self.replace_active_tetromino();

        let new_completed_lines = previously_active.lock_down(&mut self.player_screen.grid)?;
        self.record_new_completed_lines(new_completed_lines);

        self.player_screen
            .grid
            .add_garbage(self.garbage_to_be_added)?;
        self.garbage_to_be_added = 0;

        self.player_screen
            .active_tetromino
            .try_enter_grid(&self.player_screen.grid)
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
