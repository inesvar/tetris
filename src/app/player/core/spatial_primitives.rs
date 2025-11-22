//! Define `struct` [Position] and `enum` [Direction].
use super::{Deserialize, Serialize};

// y increases from top to bottom
#[cfg(test)]
pub(super) const RISE: Position = Position::new(0, -1);
pub(super) const RIGHT: Position = Position::new(1, 0);
pub(super) const FALL: Position = Position::new(0, 1);
pub(super) const LEFT: Position = Position::new(-1, 0);

/// Position on a discrete grid.
#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Debug)]
pub(in crate::app::player) struct Position {
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
    /// Default.
    North,
    East,
    South,
    West,
}

/// In [spatial_primitives](super::spatial_primitives), helpers used by [moving_primitives](super::moving_primitives)
/// to implement [ApplyRotationTranslation](super::moving_primitives::ApplyRotationTranslation) for [Position]
/// (also used by [tetromino_kind](super::tetromino_kind)).
impl Position {
    pub(in crate::app::player) const fn new(x: i32, y: i32) -> Self {
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

/// In [spatial_primitives](super::spatial_primitives), getters used by [player::render](crate::app::player::render).
impl Position {
    pub(in crate::app::player) fn x(&self) -> i32 {
        self.x
    }

    pub(in crate::app::player) fn y(&self) -> i32 {
        self.y
    }
}

/// In [spatial_primitives](super::spatial_primitives), helpers used by
/// [moving_primitives](super::moving_primitives) to implement
/// [ApplyRotationTranslation](super::moving_primitives::ApplyRotationTranslation) for [Direction].
impl Direction {
    pub(super) fn turn_clockwise(&mut self) {
        match self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West,
            Direction::West => *self = Direction::North,
        }
    }

    pub(super) fn turn_counterclockwise(&mut self) {
        match self {
            Direction::North => *self = Direction::West,
            Direction::West => *self = Direction::South,
            Direction::South => *self = Direction::East,
            Direction::East => *self = Direction::North,
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
        let mut clock = Direction::North;

        clock.turn_clockwise();
        assert_eq!(clock, Direction::East);

        clock.turn_clockwise();
        assert_eq!(clock, Direction::South);

        clock.turn_clockwise();
        assert_eq!(clock, Direction::West);
    }

    #[test]
    fn direction_turn_counterclockwise_is_correct() {
        let mut clock = Direction::North;

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::West);

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::South);

        clock.turn_counterclockwise();
        assert_eq!(clock, Direction::East);
    }

    #[test]
    fn position_add_is_correct() {
        let a = Position::new(7, -5);
        let b = Position::new(2, -4);
        let expected = Position::new(9, -9);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn position_sub_is_correct() {
        let a = Position::new(4, -3);
        let b = Position::new(-7, -4);
        let expected = Position::new(11, 1);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn position_add_assign_is_correct() {
        let mut a = Position::new(3, -1);
        let b = Position::new(-2, 6);
        let expected = Position::new(1, 5);

        a += b;

        assert_eq!(a, expected);
    }
}
