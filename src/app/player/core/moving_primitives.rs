//! Define `trait` [ApplyRotationTranslation] for [Position] and [Direction].
use super::{
    rotation_translation::{RotationTranslation, RotationType},
    spatial_primitives::{Direction, Position},
};

/// Apply a [RotationTranslation] to a spatial primitive ([Position] or [Direction]).
pub(super) trait ApplyRotationTranslation {
    /// Turn then translate as described by `movement` (turning around the center of the `movement.rotation_center` block).
    fn move_by(&mut self, movement: &RotationTranslation) {
        self.turn_around_block_center(movement);
        self.translate_by(movement);
    }
    /// Turn then translate as described by `movement` (turning around the bottom right corner of the `movement.rotation_center` block).
    fn move_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.turn_around_block_corner(movement);
        self.translate_by(movement);
    }

    /// Translate by `movement.translation`.
    fn translate_by(&mut self, movement: &RotationTranslation);
    /// Turn around the center of the `movement.rotation_center` block by `movement.rotation_type`.
    fn turn_around_block_center(&mut self, movement: &RotationTranslation);
    /// Turn around the bottom right corner of the `movement.rotation_center` block by `movement.rotation_type`.
    fn turn_around_block_corner(&mut self, movement: &RotationTranslation);
}

impl ApplyRotationTranslation for Position {
    fn translate_by(&mut self, movement: &RotationTranslation) {
        *self += movement.translation;
    }

    fn turn_around_block_center(&mut self, movement: &RotationTranslation) {
        let center: Position = movement.rotation_center;
        match movement.rotation_type {
            RotationType::Clockwise => {
                let vector = *self - center;
                *self = center + vector.turned_clockwise();
            }
            RotationType::Counterclockwise => {
                let vector = *self - center;
                *self = center + vector.turned_counterclockwise();
            }
            RotationType::HalfTurn => {
                let vector = *self - center;
                *self = center + vector.neg();
            }
            RotationType::None => {}
        }
    }

    fn turn_around_block_corner(&mut self, movement: &RotationTranslation) {
        let zoomed_rotation = movement.with_corner_rotation_center();
        let mut zoomed = self.to_zoomed_2x();

        zoomed.turn_around_block_center(&zoomed_rotation);

        *self = zoomed.from_zoomed_2x();
    }
}

impl ApplyRotationTranslation for Direction {
    fn translate_by(&mut self, _movement: &RotationTranslation) {}

    fn turn_around_block_center(&mut self, movement: &RotationTranslation) {
        match movement.rotation_type {
            RotationType::Clockwise => {
                self.turn_clockwise();
            }
            RotationType::Counterclockwise => {
                self.turn_counterclockwise();
            }
            RotationType::HalfTurn => {
                self.turn_clockwise();
                self.turn_clockwise();
            }
            RotationType::None => {}
        }
    }

    fn turn_around_block_corner(&mut self, movement: &RotationTranslation) {
        self.turn_around_block_center(movement);
    }
}

/// In [moving_primitives](super::moving_primitives), helpers to convert between regular coordinates (also refered to as "block coordinates") and 2x zoomed coordinates.
/// In 2x zoomed coordinates, block centers lie at even coordinates and block corners lie at odd coordinates.
///
/// These functions are used by [ApplyRotationTranslation] to rotate the I tetromino, which rotates around a block corner.
/// Most tetrominos rotate around a block center.
impl Position {
    /// Get 2x zoomed coordinates for the block center.
    fn to_zoomed_2x(self) -> Self {
        Self::new(2 * self.x, 2 * self.y)
    }

    /// Get 2x zoomed coordinates for the block bottom right corner.
    fn to_zoomed_2x_corner(self) -> Self {
        Self::new(2 * self.x + 1, 2 * self.y + 1)
    }

    /// Get block coordinates from 2x zoomed coordinates.
    #[allow(clippy::wrong_self_convention)]
    fn from_zoomed_2x(self) -> Self {
        Self::new(self.x.div_euclid(2), self.y.div_euclid(2))
    }
}

/// In [moving_primitives](super::moving_primitives), helper to convert between regular coordinates (also refered to as "block coordinates") and 2x zoomed coordinates.
/// In 2x zoomed coordinates, block centers lie at even coordinates and block corners lie at odd coordinates.
///
/// These functions are used by [ApplyRotationTranslation] to rotate the I tetromino, which rotates around a block corner.
/// Most tetrominos rotate around a block center.
impl RotationTranslation {
    fn with_corner_rotation_center(&self) -> Self {
        let mut copy = *self;
        copy.rotation_center = copy.rotation_center.to_zoomed_2x_corner();

        copy
    }
}

#[cfg(test)]
mod tests {
    use super::super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 0), Position::new(2, 0))]
    #[case(Position::new(0, 1), Position::new(0, 2))]
    #[case(Position::new(-1, 0), Position::new(-2, 0))]
    #[case(Position::new(0, -1), Position::new(0, -2))]
    fn to_zoomed_2x_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.to_zoomed_2x(), expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(1, 1))]
    #[case(Position::new(1, 0), Position::new(3, 1))]
    #[case(Position::new(0, 1), Position::new(1, 3))]
    #[case(Position::new(-1, 0), Position::new(-1, 1))]
    #[case(Position::new(0, -1), Position::new(1, -1))]
    fn to_zoomed_2x_corner_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.to_zoomed_2x_corner(), expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 1), Position::new(0, 0))]
    #[case(Position::new(2, 0), Position::new(1, 0))]
    #[case(Position::new(0, 2), Position::new(0, 1))]
    #[case(Position::new(3, 1), Position::new(1, 0))]
    #[case(Position::new(1, 3), Position::new(0, 1))]
    #[case(Position::new(-1, 1), Position::new(-1, 0))]
    #[case(Position::new(1, -1), Position::new(0, -1))]
    #[case(Position::new(-2, 0), Position::new(-1, 0))]
    #[case(Position::new(0, -2), Position::new(0, -1))]
    fn from_zoomed_2x_is_correct(#[case] input: Position, #[case] expected: Position) {
        assert_eq!(input.from_zoomed_2x(), expected);
    }

    const RIGHT_TRANSLATION: RotationTranslation = RotationTranslation::right();
    const CLOCKWISE_TURN: RotationTranslation =
        RotationTranslation::centered_rotation(RotationType::Clockwise);

    #[rstest]
    fn direction_translate_by_does_nothing(
        #[values(Direction::North)] expected: Direction,
        #[values(RIGHT_TRANSLATION, CLOCKWISE_TURN)] movement: RotationTranslation,
    ) {
        let mut input = expected;

        input.translate_by(&movement);

        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(Direction::North, RotationType::None, Direction::North)]
    #[case(Direction::North, RotationType::Clockwise, Direction::East)]
    #[case(Direction::North, RotationType::HalfTurn, Direction::South)]
    #[case(Direction::North, RotationType::Counterclockwise, Direction::West)]
    #[case(Direction::East, RotationType::None, Direction::East)]
    #[case(Direction::East, RotationType::Clockwise, Direction::South)]
    #[case(Direction::East, RotationType::HalfTurn, Direction::West)]
    #[case(Direction::East, RotationType::Counterclockwise, Direction::North)]
    fn direction_turn_around_functions_are_correct(
        #[case] input: Direction,
        #[case] rotation_type: RotationType,
        #[case] expected: Direction,
    ) {
        let mut actual1 = input;
        let mut actual2 = input;
        let rotation = RotationTranslation::centered_rotation(rotation_type);

        actual1.turn_around_block_center(&rotation);
        actual2.turn_around_block_corner(&rotation);

        assert_eq!(actual1, expected);
        assert_eq!(actual2, expected);
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 0), Position::new(0, 0), Position::new(1, 0))]
    #[case(Position::new(0, 1), Position::new(0, 0), Position::new(0, 1))]
    #[case(Position::new(0, 0), Position::new(1, 0), Position::new(1, 0))]
    #[case(Position::new(0, 0), Position::new(0, 1), Position::new(0, 1))]
    fn position_translate_by_is_correct(
        #[case] input: Position,
        #[case] translation: Position,
        #[case] expected: Position,
    ) {
        let mut actual = input;
        let translation = RotationTranslation::translation(translation);

        actual.translate_by(&translation);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(RISE, RotationType::None, RISE)]
    #[case(RISE, RotationType::Clockwise, RIGHT)]
    #[case(RISE, RotationType::HalfTurn, FALL)]
    #[case(RISE, RotationType::Counterclockwise, LEFT)]
    #[case(RIGHT, RotationType::None, RIGHT)]
    #[case(RIGHT, RotationType::Clockwise, FALL)]
    #[case(RIGHT, RotationType::HalfTurn, LEFT)]
    #[case(RIGHT, RotationType::Counterclockwise, RISE)]
    fn position_turn_around_is_correct(
        #[values(Position::new(0, 0), LEFT)] rotation_center: Position,
        #[case] input: Position,
        #[case] rotation_type: RotationType,
        #[case] expected: Position,
    ) {
        // Translate `input` before turning around `rotation_center`.
        let mut actual = rotation_center + input;
        let rotation = RotationTranslation::rotation(rotation_type, &rotation_center);

        actual.turn_around_block_center(&rotation);

        assert_eq!(actual, rotation_center + expected);
    }

    #[rstest]
    #[case(RISE, RotationType::None, RISE)]
    #[case(RISE, RotationType::Clockwise, Position::new(2, 0))]
    #[case(RISE, RotationType::HalfTurn, Position::new(1, 2))]
    #[case(RISE, RotationType::Counterclockwise, Position::new(-1, 1))]
    #[case(RIGHT, RotationType::None, RIGHT)]
    #[case(RIGHT, RotationType::Clockwise, Position::new(1, 1))]
    #[case(RIGHT, RotationType::HalfTurn, Position::new(0, 1))]
    #[case(RIGHT, RotationType::Counterclockwise, Position::new(0, 0))]
    fn position_turn_around_with_offset_is_correct(
        #[values(Position::new(0, 0), LEFT)] rotation_center: Position,
        #[case] input: Position,
        #[case] rotation_type: RotationType,
        #[case] expected: Position,
    ) {
        let mut actual = rotation_center + input;
        let rotation = RotationTranslation::rotation(rotation_type, &rotation_center);

        actual.turn_around_block_corner(&rotation);

        assert_eq!(actual, rotation_center + expected);
    }

    #[rstest]
    #[case(
        Position::new(0, 0),
        RotationTranslation::identity(),
        Position::new(0, 0)
    )]
    #[case(Position::new(0, 0), RotationTranslation::right(), RIGHT)]
    #[case(Position::new(0, 0), RotationTranslation::new(&Position::new(-3, 4), RotationType::Clockwise, &Position::new(-1, 0)), Position::new(-4, 5))]
    #[case(Position::new(-2, 1), RotationTranslation::new(&Position::new(-3, 4), RotationType::Clockwise, &Position::new(4, 3)), Position::new(3, 1))]
    fn move_by_for_position_is_correct(
        #[case] input: Position,
        #[case] movement: RotationTranslation,
        #[case] expected: Position,
    ) {
        let mut actual = input;

        actual.move_by(&movement);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(
        Position::new(0, 0),
        RotationTranslation::identity(),
        Position::new(0, 0)
    )]
    #[case(Position::new(0, 0), RotationTranslation::right(), RIGHT)]
    #[case(Position::new(0, 0), RotationTranslation::new(&Position::new(-3, 4), RotationType::Clockwise, &Position::new(-1, 0)), Position::new(-3, 5))]
    #[case(Position::new(-2, 1), RotationTranslation::new(&Position::new(-3, 4), RotationType::Clockwise, &Position::new(4, 3)), Position::new(4, 1))]
    fn move_by_with_offset_for_position_is_correct(
        #[case] input: Position,
        #[case] movement: RotationTranslation,
        #[case] expected: Position,
    ) {
        let mut actual = input;

        actual.move_by_with_offset(&movement);

        assert_eq!(actual, expected);
    }
}
