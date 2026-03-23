//! Define handle_key functions of [LocalPlayer].
//!
//! [handle_key_press()](LocalPlayer::handle_key_press()) is called when a key is pressed.
//! [handle_key_release()](LocalPlayer::handle_key_release()) is called when a key is released.
use super::LocalPlayer;
use core_tetris::TetrisResult;
use piston::Key;

impl LocalPlayer {
    pub fn handle_key_press(&mut self, key: Key) -> TetrisResult {
        if let Some(tetris_order) = self.keyboard.set_pressed(key) {
            self.player_screen.try_apply(tetris_order, &mut self.rng)?;
        }
        Ok(())
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}
