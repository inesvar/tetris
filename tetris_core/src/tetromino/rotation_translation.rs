//! Define `struct` [RotationTranslation] and `enum` [RotationType].
#![doc = simple_mermaid::mermaid!("rotation_translation.mmd")]

use super::Position;

/// Movement (a rotation followed by a translation).
#[derive(Clone, Copy, Default)]
pub struct RotationTranslation {
    pub rotation_type: RotationType,
    pub rotation_center: Position,
    pub translation: Position,
}

/// Four 90° rotation types.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum RotationType {
    #[default]
    /// Default.
    Identity,
    Clockwise,
    HalfTurn,
    Counterclockwise,
}

/// In [rotation_translation](super::rotation_translation), constructors used to move the [Tetromino](super::Tetromino).
impl RotationTranslation {
    pub const fn new(
        translation: &Position,
        rotation_type: RotationType,
        rotation_center: &Position,
    ) -> Self {
        Self {
            translation: *translation,
            rotation_type,
            rotation_center: *rotation_center,
        }
    }

    // TODO: remove when const traits are allowed
    pub const fn identity() -> Self {
        Self::new(
            &Position::new(0, 0),
            RotationType::Identity,
            &Position::new(0, 0),
        )
    }

    // why not by reference ?
    pub const fn translation(translation: Position) -> Self {
        Self {
            translation,
            ..Self::identity()
        }
    }
}
