//! Define `struct` [RotationTranslation] and `enum` [RotationType].
use super::{
    spatial_primitives::{FALL, LEFT, RIGHT},
    Position,
};

/// Movement composed by a rotation then a translation.
#[derive(Clone, Copy, Default)]
pub(super) struct RotationTranslation {
    pub(super) rotation_type: RotationType,
    pub(super) rotation_center: Position,
    pub(super) translation: Position,
}

/// Four rotation types.
#[derive(Clone, Copy, Default)]
pub(super) enum RotationType {
    #[default]
    None,
    Clockwise,
    HalfTurn,
    Counterclockwise,
}

impl RotationTranslation {
    pub(super) fn new(
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

    pub(super) fn translation(translation: Position) -> Self {
        Self {
            translation,
            ..Self::default()
        }
    }

    pub(super) fn fall() -> Self {
        RotationTranslation::translation(FALL)
    }

    pub(super) fn right() -> Self {
        RotationTranslation::translation(RIGHT)
    }

    pub(super) fn left() -> Self {
        RotationTranslation::translation(LEFT)
    }

    pub(super) fn rotation(rotation_type: RotationType, rotation_center: &Position) -> Self {
        Self {
            rotation_type,
            rotation_center: *rotation_center,
            ..Self::default()
        }
    }
}
