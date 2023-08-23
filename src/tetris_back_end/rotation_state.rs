//! Defines a rotation state that describes the orientation of a [Tetromino](super::Tetromino).
use serde::{Deserialize, Serialize};

/// Rotation state of a [Tetromino](super::Tetromino) among the 4 possible.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(super) enum RotationState {
    R0,
    R1,
    R2,
    R3,
}

/// Updates the rotation state.
///
/// ## Uses
/// - anytime a [Tetromino](super::Tetromino) turns
pub(super) trait RotationStateUpdate {
    /// Update the rotation state with a turn clockwise.
    fn clockwise(&mut self);

    /// Update the rotation state with a turn counterclockwise.
    fn counterclockwise(&mut self);
}

impl RotationStateUpdate for RotationState {
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
