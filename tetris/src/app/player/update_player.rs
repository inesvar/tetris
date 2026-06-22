//! Define [LocalPlayer::update] function.
use super::LocalPlayer;
use core_tetris::{GameOverError, TetrisCommand};

impl LocalPlayer {
    pub fn update(
        &mut self,
        frame_counter: u64,
        fall_speed_divide: u64,
        freeze: u64,
    ) -> Result<u32, GameOverError> {
        let mut garbage_to_send = 0;
        for command in TetrisCommand::ALL {
            if self.keyboard.should_apply_command(command) {
                garbage_to_send += self.player_screen.apply_player_move(
                    command,
                    &mut self.rng,
                    &mut self.garbage_rng,
                )?;
                if command == TetrisCommand::HardDrop {
                    self.freeze_frame = frame_counter.wrapping_sub(1);
                }
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
            garbage_to_send += self.player_screen.apply_player_move(
                TetrisCommand::HardDrop,
                &mut self.rng,
                &mut self.garbage_rng,
            )?;
        }

        self.keyboard.update();

        if self.sender {
            self.send_serialized(garbage_to_send);
        }

        Ok(garbage_to_send)
    }
}
