//! Defines rotation states of a Tetromino.
use serde::{Deserialize, Serialize};

/// Rotation state of a Tetromino among the 4 possible.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(in crate::tetris_back_end) enum RotationState {
    R0,
    R1,
    R2,
    R3,
}

/// Implementation of RotationState (used only by Tetromino).
pub(in crate::tetris_back_end) trait RotationStateImplementation {
    /// Update the RotationStatus with a turn clockwise.
    fn clockwise(&mut self);

    /// Update the RotationStatus with a turn counterclockwise.
    fn counterclockwise(&mut self);
}

impl RotationStateImplementation for RotationState {
    fn clockwise(&mut self) {
        match self {
            RotationState::R0 => {
                *self = RotationState::R1;
            }
            RotationState::R1 => {
                *self = RotationState::R2;
            }
            RotationState::R2 => {
                *self = RotationState::R3;
            }
            RotationState::R3 => {
                *self = RotationState::R0;
            }
        }
    }

    fn counterclockwise(&mut self) {
        match self {
            RotationState::R0 => {
                *self = RotationState::R3;
            }
            RotationState::R1 => {
                *self = RotationState::R0;
            }
            RotationState::R2 => {
                *self = RotationState::R1;
            }
            RotationState::R3 => {
                *self = RotationState::R2;
            }
        }
    }
}
