//! Define `struct` [Position] and `enum` [Direction].
use super::{Deserialize, Serialize};

// y increases from top to bottom
pub(super) const FALL: Position = Position::new(0, 1);
pub(super) const RISE: Position = Position::new(0, -1);
pub(super) const RIGHT: Position = Position::new(1, 0);
pub(super) const LEFT: Position = Position::new(-1, 0);

/// Position on a discrete grid, serializable.
#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Debug)]
pub(super) struct Position {
    /// Horizontal coordinate, from left to right.
    pub(super) x: i32,
    /// Vertical coordinate, *from top to bottom*.
    pub(super) y: i32,
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

impl Position {
    pub(super) const fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    // Using `const` is useful here so that well-kicks can be evaluated at compile-time in the future.
    // TODO create a const trait PositionArithmetic when it will be possible
    pub(super) const fn neg(&self) -> Self {
        Position::new(-self.x, -self.y)
    }

    pub(super) const fn mirror_x(&self) -> Self {
        Position::new(-self.x, self.y)
    }

    pub(super) const fn mirror_y(&self) -> Self {
        Position::new(self.x, -self.y)
    }

    pub(super) const fn turned_clockwise(&self) -> Self {
        Position::new(-self.y, self.x)
    }

    pub(super) const fn turned_counterclockwise(&self) -> Self {
        Position::new(self.y, -self.x)
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

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Self::Output {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_arithmetic_neg_is_correct() {
        assert_eq!(RIGHT.neg(), LEFT);
        assert_eq!(FALL.neg(), RISE);
        assert_eq!(LEFT.neg(), RIGHT);
        assert_eq!(RISE.neg(), FALL);
    }

    #[test]
    fn position_arithmetic_mirror_x_is_correct() {
        assert_eq!(RIGHT.mirror_x(), LEFT);
        assert_eq!(FALL.mirror_x(), FALL);
        assert_eq!(LEFT.mirror_x(), RIGHT);
        assert_eq!(RISE.mirror_x(), RISE);
    }

    #[test]
    fn position_arithmetic_mirror_y_is_correct() {
        assert_eq!(RIGHT.mirror_y(), RIGHT);
        assert_eq!(FALL.mirror_y(), RISE);
        assert_eq!(LEFT.mirror_y(), LEFT);
        assert_eq!(RISE.mirror_y(), FALL);
    }

    #[test]
    fn position_arithmetic_turned_clockwise_is_correct() {
        assert_eq!(RIGHT.turned_clockwise(), FALL);
        assert_eq!(FALL.turned_clockwise(), LEFT);
        assert_eq!(LEFT.turned_clockwise(), RISE);
        assert_eq!(RISE.turned_clockwise(), RIGHT);
    }

    #[test]
    fn position_arithmetic_turned_counterclockwise_is_correct() {
        assert_eq!(RIGHT.turned_counterclockwise(), RISE);
        assert_eq!(FALL.turned_counterclockwise(), RIGHT);
        assert_eq!(LEFT.turned_counterclockwise(), FALL);
        assert_eq!(RISE.turned_counterclockwise(), LEFT);
    }

    #[test]
    fn direction_turn_clockwise_is_correct() {
        let mut clock = Direction::Up;

        clock.turn_clockwise();
        assert_eq!(clock, Direction::Right);

        clock.turn_clockwise();
        assert_eq!(clock, Direction::Down);

        clock.turn_clockwise();
        assert_eq!(clock, Direction::Left);
    }

    #[test]
    fn direction_turn_counterclockwise_is_correct() {
        let mut clock = Direction::Up;

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::Left);

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::Down);

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::Right);
    }

    const TEST_VALUES: [i32; 5] = [1, -1, 0, 42, 77];

    #[test]
    fn position_add_is_correct() {
        for a in TEST_VALUES {
            for b in TEST_VALUES {
                let a_pos = Position::new(a, 0);
                let b_pos = Position::new(b, 0);

                assert_eq!((a_pos + b_pos).x, a + b);

                let a_pos = Position::new(0, a);
                let b_pos = Position::new(0, b);

                assert_eq!((a_pos + b_pos).y, a + b);
            }
        }
    }

    #[test]
    fn position_sub_is_correct() {
        for a in TEST_VALUES {
            for b in TEST_VALUES {
                let a_pos = Position::new(a, 0);
                let b_pos = Position::new(b, 0);

                assert_eq!((a_pos - b_pos).x, a - b);

                let a_pos = Position::new(0, a);
                let b_pos = Position::new(0, b);

                assert_eq!((a_pos - b_pos).y, a - b);
            }
        }
    }

    #[test]
    fn position_add_assign_is_correct() {
        for a in TEST_VALUES {
            for b in TEST_VALUES {
                let mut a_pos = Position::new(a, 0);
                a_pos += Position::new(b, 0);

                assert_eq!(a_pos.x, a + b);

                let mut a_pos = Position::new(0, a);
                a_pos += Position::new(0, b);

                assert_eq!(a_pos.y, a + b);
            }
        }
    }
}
