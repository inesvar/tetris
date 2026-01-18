//! Define `struct` [Position] and `enum` [Direction].

use serde::{Deserialize, Serialize};

/// Position on a discrete grid.
#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Position {
    /// Horizontal coordinate, from left to right.
    x: i32,
    /// Vertical coordinate, *from top to bottom*.
    y: i32,
}

impl Position {
    // y increases from top to bottom
    #[cfg(test)]
    pub const RISE: Position = Position::new(0, -1);
    pub const RIGHT: Position = Position::new(1, 0);
    pub const FALL: Position = Position::new(0, 1);
    pub const LEFT: Position = Position::new(-1, 0);
}

/// Four cardinal directions.
#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug, PartialEq)]
pub enum Direction {
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
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn neg(&self) -> Self {
        Self::new(-self.x, -self.y)
    }

    pub const fn neg_x(&self) -> Self {
        Self::new(-self.x, self.y)
    }

    pub const fn neg_y(&self) -> Self {
        Self::new(self.x, -self.y)
    }

    pub const fn turned_clockwise(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub const fn turned_counterclockwise(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn apply<F>(&self, f: F) -> Self
    where
        F: Fn(i32) -> i32,
    {
        Self::new(f(self.x), f(self.y))
    }
}

/// In [spatial_primitives](super::spatial_primitives), getters used by [player::render](crate::app::player::render).
impl Position {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

/// In [spatial_primitives](super::spatial_primitives), helpers used by
/// [moving_primitives](super::moving_primitives) to implement
/// [ApplyRotationTranslation](super::moving_primitives::ApplyRotationTranslation) for [Direction].
impl Direction {
    pub fn turn_clockwise(&mut self) {
        match self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West,
            Direction::West => *self = Direction::North,
        }
    }

    pub fn turn_counterclockwise(&mut self) {
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
    use rstest::rstest;

    #[rstest]
    #[case(Position::RISE, Position::FALL)]
    #[case(Position::RIGHT, Position::LEFT)]
    #[case(Position::FALL, Position::RISE)]
    #[case(Position::LEFT, Position::RIGHT)]
    fn position_neg_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.neg(), expected);
    }

    #[rstest]
    #[case(Position::RISE, Position::RISE)]
    #[case(Position::RIGHT, Position::LEFT)]
    #[case(Position::FALL, Position::FALL)]
    #[case(Position::LEFT, Position::RIGHT)]
    fn position_neg_x_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.neg_x(), expected);
    }

    #[rstest]
    #[case(Position::RISE, Position::FALL)]
    #[case(Position::RIGHT, Position::RIGHT)]
    #[case(Position::FALL, Position::RISE)]
    #[case(Position::LEFT, Position::LEFT)]
    fn position_neg_y_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.neg_y(), expected);
    }

    #[rstest]
    #[case(Position::RISE, Position::RIGHT)]
    #[case(Position::RIGHT, Position::FALL)]
    #[case(Position::FALL, Position::LEFT)]
    #[case(Position::LEFT, Position::RISE)]
    fn position_turned_clockwise_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.turned_clockwise(), expected);
    }

    #[rstest]
    #[case(Position::RISE, Position::LEFT)]
    #[case(Position::RIGHT, Position::RISE)]
    #[case(Position::FALL, Position::RIGHT)]
    #[case(Position::LEFT, Position::FALL)]
    fn position_turned_counterclockwise_is_correct(
        #[case] input: Position,
        #[case] expected: Position,
    ) {
        assert_eq!(input.turned_counterclockwise(), expected);
    }

    #[rstest]
    #[case(Direction::North, Direction::East)]
    #[case(Direction::East, Direction::South)]
    #[case(Direction::South, Direction::West)]
    #[case(Direction::West, Direction::North)]
    fn direction_turn_clockwise_is_correct(#[case] input: Direction, #[case] expected: Direction) {
        let mut actual = input;

        actual.turn_clockwise();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Direction::North, Direction::West)]
    #[case(Direction::East, Direction::North)]
    #[case(Direction::South, Direction::East)]
    #[case(Direction::West, Direction::South)]
    fn direction_turn_counterclockwise_is_correct(
        #[case] input: Direction,
        #[case] expected: Direction,
    ) {
        let mut actual = input;

        actual.turn_counterclockwise();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 0), Position::new(0, 0), Position::new(1, 0))]
    #[case(Position::new(0, 1), Position::new(0, 0), Position::new(0, 1))]
    #[case(Position::new(0, 0), Position::new(1, 0), Position::new(1, 0))]
    #[case(Position::new(0, 0), Position::new(0, 1), Position::new(0, 1))]
    fn position_add_is_correct(
        #[case] op1: Position,
        #[case] op2: Position,
        #[case] expected: Position,
    ) {
        assert_eq!(op1 + op2, expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 0), Position::new(0, 0), Position::new(1, 0))]
    #[case(Position::new(0, 1), Position::new(0, 0), Position::new(0, 1))]
    #[case(Position::new(0, 0), Position::new(1, 0), Position::new(-1, 0))]
    #[case(Position::new(0, 0), Position::new(0, 1), Position::new(0, -1))]
    fn position_sub_is_correct(
        #[case] op1: Position,
        #[case] op2: Position,
        #[case] expected: Position,
    ) {
        assert_eq!(op1 - op2, expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 0), Position::new(0, 0), Position::new(1, 0))]
    #[case(Position::new(0, 1), Position::new(0, 0), Position::new(0, 1))]
    #[case(Position::new(0, 0), Position::new(1, 0), Position::new(1, 0))]
    #[case(Position::new(0, 0), Position::new(0, 1), Position::new(0, 1))]
    fn position_add_assign_is_correct(
        #[case] op1: Position,
        #[case] op2: Position,
        #[case] expected: Position,
    ) {
        let mut actual = op1;

        actual += op2;

        assert_eq!(actual, expected);
    }
}
