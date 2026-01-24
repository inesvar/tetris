//! Define handle_key functions of [LocalPlayer].
//!
//! [handle_key_press()](LocalPlayer::handle_key_press()) is called when a key is pressed.
//! [handle_key_release()](LocalPlayer::handle_key_release()) is called when a key is released.
use tetris_core::TetrominoMove;
use super::LocalPlayer;
use crate::{
    app::{GameFlowChange, RunningState},
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
        /******************************
         *       STARTING GAME        *
         ******************************/
        // the starting game doesn't listen to anything
        if running == RunningState::Starting {
            return GameFlowChange::NoChange;
        }

        self.keyboard.set_pressed(key);

        /******************************
         *       UNACTIVE GAME        *
         ******************************/

        // the unactive game only listens to the RESTART_KEYS
        if running == RunningState::NotRunning {
            if self.keyboard.was_just_pressed(&RESTART_KEYS) {
                return GameFlowChange::Restart;
            } else {
                return GameFlowChange::NoChange;
            }
        }

        /******************************
         * (ABOUT TO BE) PAUSED GAME  *
         ******************************/

        // the paused game only listens to the PAUSE_KEYS
        if running == RunningState::Paused {
            if self.keyboard.was_just_pressed(&PAUSE_KEYS) {
                return GameFlowChange::Resume;
            } else {
                return GameFlowChange::NoChange;
            }
        // the game pauses if PAUSE_KEYS are pressed
        } else if running == RunningState::Running && self.keyboard.was_just_pressed(&PAUSE_KEYS) {
            return GameFlowChange::Pause;
        }

        /******************************
         *         ACTIVE GAME        *
         ******************************/
        self.move_active_tetromino(keybindings)
    }

    fn move_active_tetromino(&mut self, keybindings: &Keybindings) -> GameFlowChange {
        if self
            .keyboard
            .was_just_pressed(&keybindings.hold_tetromino_keys)
        {
            // hold the tetromino
            if let Some(mut saved) = self.player_screen.saved_tetromino {
                self.player_screen.active_tetromino.reset();

                std::mem::swap(&mut saved, &mut self.player_screen.active_tetromino);
                self.player_screen.saved_tetromino = Some(saved);
            } else {
                self.player_screen.active_tetromino.reset();

                self.player_screen.saved_tetromino = Some(self.player_screen.active_tetromino);
                self.get_new_tetromino();
            }
        }

        // Pressed once events
        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_clockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::Clockwise, &self.player_screen.grid);
        }
        // it's not an if else in case the player put the same keybindings for both clock and counter...
        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_counterclockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::Counterclockwise, &self.player_screen.grid);
        }

        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_half_turn_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::HalfTurn, &self.player_screen.grid);
        }

        // move the tetromino left or right
        if self.keyboard.was_just_pressed(&keybindings.left_keys) {
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::Left, &self.player_screen.grid);
        }
        // it's not an if else in case the player put the same keybindings for both left and right...
        if self.keyboard.was_just_pressed(&keybindings.right_keys) {
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::Right, &self.player_screen.grid);
        }

        if self.keyboard.was_just_pressed(&keybindings.hard_drop_keys) {
            // hard drop the tetromino
            self.player_screen
                .active_tetromino
                .apply(TetrominoMove::HardDrop, &self.player_screen.grid);
            if self.lock_down_tetromino().is_err() {
                return GameFlowChange::GameOver;
            }
        }

        GameFlowChange::NoChange
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}
