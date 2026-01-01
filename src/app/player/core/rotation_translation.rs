//! Define `struct` [RotationTranslation] and `enum` [RotationType].
use super::spatial_primitives::Position;

/// Movement (a rotation followed by a translation).
#[derive(Clone, Copy, Default)]
pub(super) struct RotationTranslation {
    pub(super) rotation_type: RotationType,
    pub(super) rotation_center: Position,
    pub(super) translation: Position,
}

/// Four 90° rotation types.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub(super) enum RotationType {
    #[default]
    /// Default.
    Identity,
    Clockwise,
    HalfTurn,
    Counterclockwise,
}

/// In [rotation_translation](super::rotation_translation), constructors used to move the [Tetromino](super::Tetromino).
impl RotationTranslation {
    pub(super) const fn new(
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
    pub(super) const fn identity() -> Self {
        Self::new(
            &Position::new(0, 0),
            RotationType::Identity,
            &Position::new(0, 0),
        )
    }

    // why not by reference ?
    pub(super) const fn translation(translation: Position) -> Self {
        Self {
            translation,
            ..Self::identity()
        }
    }
}
