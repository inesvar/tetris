//! Define `struct` [RotationTranslation], `enum` [RotationType] and `enum` [Direction].
use super::{Deserialize, Position, Serialize};

/// Movement composed by a rotation then a translation.
#[derive(Default)]
pub(super) struct RotationTranslation {
    pub(super) rotation_type: RotationType,
    pub(super) rotation_center: Position,
    pub(super) translation: Position,
}

/// Four rotation types.
#[derive(Clone, Copy, Default)]
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
        *self as usize
    }
}

impl Direction {
    fn to_usize(&self) -> usize {
        *self as usize
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
    // Why not implementing Add ? or TryInto ?
    pub(super) fn update(&mut self, rotation_type: &RotationType) {
        *self = Self::ALL_VARIANTS[self.to_usize() + rotation_type.to_usize()]
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, RotationType};

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
}
