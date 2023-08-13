//! Defines composite movements needed to describe how a tetromino moves.
use super::point::Point;

/// Movements composed by a translation, then a rotation.
pub struct TranslationRotation {
    pub(in crate::tetris_back_end) translation: Point,
    pub(in crate::tetris_back_end) rotation: Rotation,
}

/// Rotation types of a Tetromino.
pub(in crate::tetris_back_end) enum Rotation {
    Clockwise(Point),
    Counterclockwise(Point),
    None,
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

    /// Returns a composite movement, translation then rotation.
    pub(in crate::tetris_back_end) fn new(translation: Point, rotation: Rotation) -> Self {
        TranslationRotation {
            translation,
            rotation,
        }
    }

    /// Returns a translation movement.
    pub(in crate::tetris_back_end) fn translation(translation: Point) -> Self {
        TranslationRotation {
            translation,
            rotation: Rotation::None,
        }
    }

    /// Returns a translation one cell to the right.
    pub(in crate::tetris_back_end) fn right() -> Self {
        TranslationRotation::translation(Point::new(1, 0))
    }

    /// Returns a translation one cell to the left.
    pub(in crate::tetris_back_end) fn left() -> Self {
        TranslationRotation::translation(Point::new(-1, 0))
    }
}
