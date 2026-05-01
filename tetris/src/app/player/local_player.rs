//! Define the general implementation of [LocalPlayer].
use super::{input_commands::InputCommands, LocalPlayer, TetrisPlayer};
use crate::{
    app::{remote::OutboundMessage, PlayerConfig},
    once,
    settings::BAG_TYPE,
};
use piston::Key;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::net::TcpStream;
use ui_tetris::Keybindings;

impl LocalPlayer {
    pub fn new(player_config: &PlayerConfig, keybindings: Keybindings) -> Self {
        let mut rng = Pcg32::seed_from_u64(0);
        let garbage_rng = Pcg32::seed_from_u64(0);

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

        let player_screen = TetrisPlayer::default(&mut rng, BAG_TYPE);

        LocalPlayer {
            player_screen,
            keyboard: InputCommands::new(keybindings),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            sender,
            remote_ip,
            rng,
            garbage_rng,
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        self.keyboard.set_pressed(key);
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }

    pub fn get_player(&self) -> &TetrisPlayer {
        &self.player_screen
    }

    pub fn get_keybindings(&mut self) -> Keybindings {
        self.keyboard.get_keybindings()
    }

    pub fn set_new_keybindings(&mut self, keybindings: &Keybindings) {
        self.keyboard.set_new_keybindings(keybindings);
    }

    pub fn reset(&mut self, seed: u64) {
        self.rng = Pcg32::seed_from_u64(seed);
        self.player_screen = TetrisPlayer::default(&mut self.rng, self.player_screen.bag_type());
        self.freeze_frame = 0;
    }

    pub fn push_garbage(&mut self, completed_lines: u64) {
        self.player_screen.push_garbage(completed_lines);
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        self.player_screen.get_lines_completed()
    }

    pub(in crate::app) fn score(&self) -> u64 {
        self.player_screen.score()
    }
}

impl LocalPlayer {
    /// Sends the player screen to the remote player and resets the new_completed_lines attribute.
    pub(in crate::app) fn send_serialized(&mut self) {
        if let Ok(stream) = TcpStream::connect(&self.remote_ip) {
            serde_cbor::to_writer::<TcpStream, OutboundMessage>(
                stream,
                &OutboundMessage::TetrisPlayer(&self.player_screen),
            )
            .unwrap();
        }
        once!("sent serialized data to the remote");
        // Set the number of completed lines to 0
        if self.player_screen.new_completed_lines() != 0 {
            once!(
                "the {} completed lines were sent to the adversary and they were reset to 0",
                self.player_screen.new_completed_lines()
            );
            *self.player_screen.new_completed_lines_mut() = 0;
        }
    }
}
