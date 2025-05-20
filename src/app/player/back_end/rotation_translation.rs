//! Define `struct` [RotationTranslation], `enum` [RotationType] and `enum` [Direction].
use super::{
    spatial_primitives::{FALL, LEFT, RIGHT, RISE},
    Deserialize, Position, Serialize,
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
    #[allow(dead_code)] // TODO
    HalfTurn,
    Counterclockwise,
}

/// Four cardinal directions.
#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
pub(super) enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
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
}

impl Direction {
    pub(super) fn turn_clockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    pub(super) fn turn_counterclockwise(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
            Direction::Right => *self = Direction::Up,
        }
    }
}

impl From<Direction> for Position {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => RISE,
            Direction::Right => RIGHT,
            Direction::Down => FALL,
            Direction::Left => LEFT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use enum_iterator::{next_cycle, previous_cycle};

    #[test]
    fn direction_update() {
        let mut direction = Direction::Up;
        let mut direction_copy = direction;

        for _ in 0..4 {
            direction.turn_clockwise();
            direction_copy = next_cycle(&direction_copy);
            assert_eq!(direction, direction_copy);
        }

        for _ in 0..4 {
            direction.turn_counterclockwise();
            direction_copy = previous_cycle(&direction_copy);
            assert_eq!(direction, direction_copy);
        }
    }
}
