//! Defines composite movements needed to describe how a tetromino moves.
use super::{point::Point, TranslationRotation};

/// Rotation movement.
pub(super) enum Rotation {
    Clockwise(Point),
    Counterclockwise(Point),
    None,
}

/// Rotation types.
pub(super) enum RotationType {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    /// Constructor for non-null rotation movements.
    fn new(rtype: RotationType, center: Point) -> Self {
        match rtype {
            RotationType::Clockwise => Rotation::Clockwise(center),
            RotationType::Counterclockwise => Rotation::Counterclockwise(center),
        }
    }
}

impl TranslationRotation {
    /// Returns a null movement.
    pub fn null() -> Self {
        TranslationRotation {
            translation: Point::new(0, 0),
            rotation: Rotation::None,
        }
    }

    /// Returns a translation one cell towards the bottom.
    pub fn fall() -> Self {
        TranslationRotation::translation(Point::new(0, 1))
    }

    /// Returns a composite movement, translation then rotation (around the translated center).
    /// For a pure translation, use translation method.
    pub(super) fn new(translation: Point, rtype: RotationType, center: &Point) -> Self {
        TranslationRotation {
            translation,
            // the rotation center is the center of the struct translated by translation
            rotation: Rotation::new(rtype, *center + translation),
        }
    }

    /// Returns a translation movement.
    pub(super) fn translation(translation: Point) -> Self {
        TranslationRotation {
            translation,
            rotation: Rotation::None,
        }
    }

    /// Returns a translation one cell to the right.
    pub(super) fn right() -> Self {
        TranslationRotation::translation(Point::new(1, 0))
    }

    /// Returns a translation one cell to the left.
    pub(super) fn left() -> Self {
        TranslationRotation::translation(Point::new(-1, 0))
    }
}
