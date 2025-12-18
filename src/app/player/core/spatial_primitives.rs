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
    use rstest::rstest;

    #[rstest]
    #[case(RISE, FALL)]
    #[case(RIGHT, LEFT)]
    #[case(FALL, RISE)]
    #[case(LEFT, RIGHT)]
    fn position_neg_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.neg(), expected);
    }

    #[rstest]
    #[case(RISE, RISE)]
    #[case(RIGHT, LEFT)]
    #[case(FALL, FALL)]
    #[case(LEFT, RIGHT)]
    fn position_mirror_x_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.mirror_x(), expected);
    }

    #[rstest]
    #[case(RISE, FALL)]
    #[case(RIGHT, RIGHT)]
    #[case(FALL, RISE)]
    #[case(LEFT, LEFT)]
    fn position_mirror_y_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.mirror_y(), expected);
    }

    #[rstest]
    #[case(RISE, RIGHT)]
    #[case(RIGHT, FALL)]
    #[case(FALL, LEFT)]
    #[case(LEFT, RISE)]
    fn position_turned_clockwise_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.turned_clockwise(), expected);
    }

    #[rstest]
    #[case(RISE, LEFT)]
    #[case(RIGHT, RISE)]
    #[case(FALL, RIGHT)]
    #[case(LEFT, FALL)]
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
