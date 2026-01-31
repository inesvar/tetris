//! Define handle_key functions of [LocalPlayer].
//!
//! [handle_key_press()](LocalPlayer::handle_key_press()) is called when a key is pressed.
//! [handle_key_release()](LocalPlayer::handle_key_release()) is called when a key is released.
use super::LocalPlayer;
use crate::{
    app::{GameFlowChange, RunningState},
    settings::{Keybindings, PAUSE_KEYS, RESTART_KEYS},
};
use piston::Key;
use tetris_core::{TetrisOrder, TetrisResult, TetrominoMove};

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
        if self.move_active_tetromino(keybindings).is_err() {
            GameFlowChange::GameOver
        } else {
            GameFlowChange::NoChange
        }
    }

    fn move_active_tetromino(&mut self, keybindings: &Keybindings) -> TetrisResult {
        if self
            .keyboard
            .was_just_pressed(&keybindings.hold_tetromino_keys)
        {
            self.player_screen
                .try_apply(TetrisOrder::PlayerStashesTetromino, &mut self.rng)?;
        }

        // Pressed once events
        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_clockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .try_apply(TetrominoMove::Clockwise.into(), &mut self.rng)?;
        }
        // it's not an if else in case the player put the same keybindings for both clock and counter...
        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_counterclockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .try_apply(TetrominoMove::Counterclockwise.into(), &mut self.rng)?;
        }

        if self
            .keyboard
            .was_just_pressed(&keybindings.rotate_half_turn_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .try_apply(TetrominoMove::HalfTurn.into(), &mut self.rng)?;
        }

        // move the tetromino left or right
        if self.keyboard.was_just_pressed(&keybindings.left_keys) {
            self.player_screen
                .try_apply(TetrominoMove::Left.into(), &mut self.rng)?;
        }
        // it's not an if else in case the player put the same keybindings for both left and right...
        if self.keyboard.was_just_pressed(&keybindings.right_keys) {
            self.player_screen
                .try_apply(TetrominoMove::Right.into(), &mut self.rng)?;
        }

        if self.keyboard.was_just_pressed(&keybindings.hard_drop_keys) {
            // hard drop the tetromino
            self.player_screen
                .try_apply(TetrominoMove::HardDrop.into(), &mut self.rng)?;
            self.player_screen
                .try_apply(TetrisOrder::TetrominoLocksDown, &mut self.rng)?;
        }

        Ok(())
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}
