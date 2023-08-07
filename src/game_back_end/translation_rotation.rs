//! Defines different types of movements needed for Tetromino.
use super::point::Point;
/// Common types of movement of a Tetromino.
pub struct TranslationRotation {
    pub translation: Point,
    pub rotation: Rotation,
}

/// Rotation types of a Tetromino.
pub enum Rotation {
    Clockwise(Point),
    Counterclockwise(Point),
    None,
}

impl TranslationRotation {
    pub fn new(translation: Point, rotation: Rotation) -> Self {
        TranslationRotation {
            translation,
            rotation,
        }
    }

    pub fn null() -> Self {
        TranslationRotation {
            translation: Point::new(0, 0),
            rotation: Rotation::None,
        }
    }

    pub fn translation(translation: Point) -> Self {
        TranslationRotation {
            translation,
            rotation: Rotation::None,
        }
    }

    pub fn fall() -> Self {
        TranslationRotation::translation(Point::new(0, 1))
    }

    pub fn right() -> Self {
        TranslationRotation::translation(Point::new(1, 0))
    }

    pub fn left() -> Self {
        TranslationRotation::translation(Point::new(-1, 0))
    }
}
