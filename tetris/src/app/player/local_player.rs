//! Define the general implementation of [LocalPlayer].
use super::{pressed_keys::PressedKeys, LocalPlayer, TetrisPlayer};
use crate::{
    app::{Countdown, PlayerConfig},
    once,
};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;
use tetris_core::TetrisResult;

impl LocalPlayer {
    pub fn new(player_config: &PlayerConfig) -> Self {
        let mut rng = Pcg32::seed_from_u64(0);

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

        let player_screen = TetrisPlayer::new(&mut rng);

        LocalPlayer {
            player_screen,
            keyboard: PressedKeys::new(),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            sender,
            remote_ip,
            garbage_to_be_added: 0,
            rng,
        }
    }

    pub fn reset(&mut self, seed: u64) {
        self.rng = Pcg32::seed_from_u64(seed);
        self.player_screen = TetrisPlayer::new(&mut self.rng);
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
        let _ = self
            .player_screen
            .active_tetromino
            .try_enter_grid(&self.player_screen.grid);
    }

    pub(in crate::app) fn countdown(&mut self, i: &Countdown) {
        match i {
            Countdown::One => self
                .player_screen
                .grid
                .one(self.player_screen.active_tetromino.color()),
            Countdown::Two => self
                .player_screen
                .grid
                .two(self.player_screen.active_tetromino.color()),
            Countdown::Three => self
                .player_screen
                .grid
                .three(self.player_screen.active_tetromino.color()),
        }
    }
}

impl LocalPlayer {
    pub(super) fn stash(&mut self) -> TetrisResult {
        let mut previously_active = self
            .player_screen
            .replace_active_tetromino_using_stash(&mut self.rng);
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

    pub(super) fn lock_down(&mut self) -> TetrisResult {
        let previously_active = self.player_screen.replace_active_tetromino(&mut self.rng);

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
            serde_cbor::to_writer::<TcpStream, TetrisPlayer>(stream, &self.player_screen).unwrap();
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
