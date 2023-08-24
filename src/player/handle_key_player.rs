//! Defines handle_key functions of [LocalPlayer].
//! 
//! [handle_key_press()](LocalPlayer::handle_key_press()) is called when a key is pressed.
//! [handle_key_release()](LocalPlayer::handle_key_release()) is called when a key is released.
use super::{GameFlowChange, LocalPlayer};
use crate::{
    app::RunningState,
    settings::{Keybindings, PAUSE_KEYS, RESTART_KEYS},
};
use piston::Key;

impl LocalPlayer {
    /// handle_key_press is called when a key is pressed.
    ///
    /// It moves the tetromino accordingly if needed, it's responsible for all tetromino events except for the following (which are handled in update) :
    /// - the tetromino "falling" down naturally
    /// - the tetromino "freezing" at the bottom
    /// - the tetromino moving continuously to the right (resp. left) on a long key press
    ///
    /// Its also responsible for the events :
    /// - pause
    /// - restart
    pub fn handle_key_press(
        &mut self,
        keybindings: &Keybindings,
        key: Key,
        running: RunningState,
    ) -> GameFlowChange {
        self.keyboard.set_pressed(key);

        /******************************
         *       UNACTIVE GAME        *
         ******************************/

        // the unactive game only listens to the RESTART_KEYS
        if running == RunningState::NotRunning {
            if self.keyboard.is_any_last_pressed(&RESTART_KEYS) {
                return GameFlowChange::Restart;
            } else {
                return GameFlowChange::Other;
            }
        }

        /******************************
         * (ABOUT TO BE) PAUSED GAME  *
         ******************************/

        // the paused game only listens to the PAUSE_KEYS
        if running == RunningState::Paused {
            if self.keyboard.is_any_last_pressed(&PAUSE_KEYS) {
                return GameFlowChange::Resume;
            } else {
                return GameFlowChange::Other;
            }
        // the game pauses if PAUSE_KEYS are pressed
        } else if running == RunningState::Running && self.keyboard.is_any_last_pressed(&PAUSE_KEYS)
        {
            return GameFlowChange::Pause;
        }

        /******************************
         *         ACTIVE GAME        *
         ******************************/

        // Pressed once events
        if self
            .keyboard
            .is_any_last_pressed(&keybindings.rotate_clockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_clockwise(&self.player_screen.grid.matrix);
        }
        // it's not an if else in case the player put the same keybindings for both clock and counter...
        if self
            .keyboard
            .is_any_last_pressed(&keybindings.rotate_counterclockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_counterclockwise(&self.player_screen.grid.matrix);
        }

        if self
            .keyboard
            .is_any_last_pressed(&keybindings.hold_tetromino_keys)
        {
            // hold the tetromino
            if let Some(mut saved) = self.player_screen.saved_tetromino {
                self.player_screen.active_tetromino.reset_position();

                std::mem::swap(&mut saved, &mut self.player_screen.active_tetromino);
                self.player_screen.saved_tetromino = Some(saved);
            } else {
                self.player_screen.active_tetromino.reset_position();

                self.player_screen.saved_tetromino = Some(self.player_screen.active_tetromino);
                self.get_new_tetromino();
            }
        }

        // move the tetromino left or right
        if self.keyboard.is_any_last_pressed(&keybindings.left_keys) {
            self.player_screen
                .active_tetromino
                .left(&self.player_screen.grid.matrix);
        }
        // it's not an if else in case the player put the same keybindings for both left and right...
        if self.keyboard.is_any_last_pressed(&keybindings.right_keys) {
            self.player_screen
                .active_tetromino
                .right(&self.player_screen.grid.matrix);
        }

        if self
            .keyboard
            .is_any_last_pressed(&keybindings.hard_drop_keys)
        {
            // hard drop the tetromino
            self.player_screen
                .active_tetromino
                .hard_drop(&self.player_screen.grid.matrix);
            match self
                .player_screen
                .grid
                .freeze_tetromino(&mut self.player_screen.active_tetromino)
            {
                Some(completed_lines) => {
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
                None => self.declare_game_over(),
            }
        }
        GameFlowChange::Other
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}
