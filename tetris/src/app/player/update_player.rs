//! Define [LocalPlayer::update] function.
use super::LocalPlayer;
use core_tetris::{TetrisCommand, TetrisResult};

impl LocalPlayer {
    pub fn update(
        &mut self,
        frame_counter: u64,
        fall_speed_divide: u64,
        freeze: u64,
    ) -> TetrisResult {
        for command in TetrisCommand::ALL {
            if self.keyboard.should_apply_command(command) {
                self.player_screen.apply_player_move(
                    command,
                    &mut self.rng,
                    &mut self.garbage_rng,
                )?;
            }
        }

        if frame_counter % fall_speed_divide == 0
            && !self.player_screen.apply_gravity()
            && self.freeze_frame < frame_counter
        {
            // if the tetromino just reached the bottom, update the freeze_frame
            self.freeze_frame = frame_counter + freeze;
        }

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame && !self.player_screen.apply_gravity() {
            self.player_screen.apply_player_move(
                TetrisCommand::HardDrop,
                &mut self.rng,
                &mut self.garbage_rng,
            )?;
        }

        self.keyboard.update();

        if self.sender {
            self.send_serialized();
        }

        Ok(())
    }
}
