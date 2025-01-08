//! Implements `struct` [TranslationRotation], defines `struct` [Rotation], `enum` [RotationType] and `enum` [Direction].
use super::{spatial_primitives::Position, TranslationRotation};
use serde::{Deserialize, Serialize};

/// Rotation of 90° around a center.
pub(super) struct Rotation {
    pub(super) rotation_type: RotationType,
    pub(super) center: Position,
}

/// 90° rotation types.
#[derive(Clone)]
pub(super) enum RotationType {
    None = 0, // TODO remove this variant
    Clockwise = 1,
    #[allow(dead_code)]
    HalfTurn = 2,
    Counterclockwise = 3,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub(super) enum Direction {
    #[default]
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Rotation {
    /// Constructor for non-null rotation movements.
    fn new(rotation_type: RotationType, center: Position) -> Self {
        Rotation {
            rotation_type,
            center,
        }
    }

    // TODO remove this
    fn null() -> Self {
        Rotation::new(RotationType::None, Position::default())
    }
}

impl TranslationRotation {
    // TODO remove this
    /// Returns a null movement.
    pub(in crate::app::player) fn null() -> Self {
        TranslationRotation {
            translation: Position::default(),
            rotation: Rotation::null(),
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
            rotation: Rotation::null(),
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

impl Direction {
    fn to_usize(self) -> usize {
        self as usize
    }

    const ALL_VARIANTS: [Direction; 8] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    // TODO maybe implement ApplyTranslationRotation for Direction?
    pub(super) fn update(&mut self, rotation_type: &RotationType) {
        *self = Self::ALL_VARIANTS[self.to_usize() + rotation_type.to_usize()]
    }
}

impl RotationType {
    fn to_usize(&self) -> usize {
        self.clone() as usize
    }
}
