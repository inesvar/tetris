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
        for command in TetrisCommand::ALL_EXCEPT_FALL {
            if self.keyboard.should_apply_command(command) {
                self.player_screen
                    .try_apply(command, &mut self.rng, &mut self.garbage_rng)?;
            }
        }

        if self.should_apply_fall(frame_counter, fall_speed_divide)
            && !self.player_screen.try_fall()
            && self.freeze_frame < frame_counter
        {
            // if the tetromino just reached the bottom, update the freeze_frame
            self.freeze_frame = frame_counter + freeze;
        }

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame && !self.player_screen.try_fall() {
            self.player_screen.try_apply(
                TetrominoMove::HardDrop.into(),
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

    fn should_apply_fall(&self, frame_counter: u64, fall_speed_divide: u64) -> bool {
        self.keyboard
            .should_apply_command(TetrominoMove::Fall.into())
            || frame_counter % fall_speed_divide == 0
    }
}
