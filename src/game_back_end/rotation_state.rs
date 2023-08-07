//! Defines the 4 rotation states of a Tetromino.
use serde::{Deserialize, Serialize};

/// Rotation state of a Tetromino among the 4 possible.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum RotationState {
    R0,
    R1,
    R2,
    R3,
}

impl RotationState {
    pub fn clockwise(&mut self) {
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

    pub fn counterclockwise(&mut self) {
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
