//! Define [LocalPlayer::update] function.
use super::LocalPlayer;
use core_tetris::{TetrisCommand, TetrisResult, TetrominoMove};

impl LocalPlayer {
    pub fn update(
        &mut self,
        frame_counter: u64,
        fall_speed_divide: u64,
        freeze: u64,
    ) -> TetrisResult {
        for command in TetrisCommand::ALL {
            if self.keyboard.should_apply_command(command) {
                let command_succeeded = self.player_screen.try_apply(command, &mut self.rng)?;
                if !command_succeeded && self.freeze_frame < frame_counter {
                    // if the tetromino reaches the bottom, set the freeze_frame
                    self.freeze_frame = frame_counter + freeze;
                }
            }
        }

        // move the tetromino down to emulate its fall
        if frame_counter % fall_speed_divide == 0
            && !self
                .player_screen
                .try_apply(TetrominoMove::Fall.into(), &mut self.rng)?
            && self.freeze_frame < frame_counter
        {
            // if the tetromino reaches the bottom, set the freeze_frame
            self.freeze_frame = frame_counter + freeze;
        }

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame
            && !self
                .player_screen
                .try_apply(TetrominoMove::Fall.into(), &mut self.rng)?
        {
            self.player_screen
                .try_apply(TetrisCommand::LockDown, &mut self.rng)?;
        }

        // Updates the time for the keyboard
        self.keyboard.update();

        // Send the player_screen data if necessary
        if self.sender {
            self.send_serialized();
        }

        Ok(())
    }
}
