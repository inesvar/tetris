//! Define `struct` [RotationTranslation], `enum` [RotationType] and `enum` [Direction].
use super::{Deserialize, Position, Serialize};

/// Movements composed by a 90° rotation then a translation.
pub(super) struct RotationTranslation {
    pub(super) translation: Position,
    pub(super) rotation_type: RotationType,
    pub(super) rotation_center: Position,
}

/// 90° rotation types.
#[derive(Clone, Default)]
pub(super) enum RotationType {
    #[default]
    None = 0,
    Clockwise = 1,
    #[allow(dead_code)] // TODO
    HalfTurn = 2,
    Counterclockwise = 3,
}

/// Four cardinal directions.
#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug, PartialEq)]
pub(super) enum Direction {
    #[default]
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl RotationTranslation {
    pub(super) fn new(
        translation: &Position,
        rotation_type: RotationType,
        center: &Position,
    ) -> Self {
        RotationTranslation {
            translation: *translation,
            rotation_type,
            // the rotation center is the center of the struct translated by translation
            rotation_center: *center,
        }
    }

    pub(super) fn translation(translation: Position) -> Self {
        RotationTranslation {
            translation,
            rotation_type: RotationType::default(),
            rotation_center: Position::default(),
        }
    }

    pub(super) fn fall() -> Self {
        RotationTranslation::translation(Position::new(0, 1))
    }

    pub(super) fn right() -> Self {
        RotationTranslation::translation(Position::new(1, 0))
    }

    pub(super) fn left() -> Self {
        RotationTranslation::translation(Position::new(-1, 0))
    }
}

impl RotationType {
    fn to_usize(&self) -> usize {
        self.clone() as usize
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

    // TODO maybe implement ApplyRotationTranslation for Direction?
    pub(super) fn update(&mut self, rotation_type: &RotationType) {
        *self = Self::ALL_VARIANTS[self.to_usize() + rotation_type.to_usize()]
    }
}

#[test]
fn direction_update() {
    let clockwise_turn = RotationType::Clockwise;
    let half_turn = RotationType::HalfTurn;
    let counterclockwise_turn = RotationType::Counterclockwise;

    let mut direction = Direction::Up;
    direction.update(&clockwise_turn);
    assert_eq!(direction, Direction::Right);

    direction.update(&half_turn);
    assert_eq!(direction, Direction::Left);

    direction.update(&counterclockwise_turn);
    assert_eq!(direction, Direction::Down);
}
