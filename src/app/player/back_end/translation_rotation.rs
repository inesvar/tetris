//! Implements `struct` [TranslationRotation], defines `enum` [Rotation]
//! and `enum` [RotationType].
use super::{spatial_primitives::Position, TranslationRotation};

/// .
pub(super) enum Rotation {
    Clockwise(Position),
    Counterclockwise(Position),
    HalfTurn(Position),
    NoRotation,
}

/// Rotation types.
pub(super) enum RotationType {
    Clockwise,
    Counterclockwise,
    #[allow(dead_code)]
    HalfTurn,
}

impl Rotation {
    /// Constructor for non-null rotation movements.
    fn new(rotation_type: RotationType, center: Position) -> Self {
        match rotation_type {
            RotationType::Clockwise => Rotation::Clockwise(center),
            RotationType::Counterclockwise => Rotation::Counterclockwise(center),
            RotationType::HalfTurn => Rotation::HalfTurn(center),
        }
    }
}

impl TranslationRotation {
    // TODO make this pub(super)
    /// Returns a null movement.
    pub(in crate::app::player) fn null() -> Self {
        TranslationRotation {
            translation: Position::new(0, 0),
            rotation: Rotation::NoRotation,
        }
    }

    /// Returns a translation one cell towards the bottom.
    pub(super) fn fall() -> Self {
        TranslationRotation::translation(Position::new(0, 1))
    }

    /// Returns a composite movement, translation then rotation (around the translated center).
    /// For a pure translation, use translation method.
    pub(super) fn new(translation: &Position, rtype: RotationType, center: &Position) -> Self {
        TranslationRotation {
            translation: *translation,
            // the rotation center is the center of the struct translated by translation
            rotation: Rotation::new(rtype, center + translation),
        }
    }

    /// Returns a translation movement.
    pub(super) fn translation(translation: Position) -> Self {
        TranslationRotation {
            translation,
            rotation: Rotation::NoRotation,
        }
    }

    /// Returns a translation one cell to the right.
    pub(super) fn right() -> Self {
        TranslationRotation::translation(Position::new(1, 0))
    }

    /// Returns a translation one cell to the left.
    pub(super) fn left() -> Self {
        TranslationRotation::translation(Position::new(-1, 0))
    }
}
