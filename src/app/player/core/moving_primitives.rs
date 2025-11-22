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
        let mut zoomed = self.get_block_center_coordinates();
        let mut zoomed_rotation = *movement;
        zoomed_rotation.rotation_center = zoomed_rotation
            .rotation_center
            .get_block_bottom_right_coordinates();
        zoomed.turn_around_block_center(&zoomed_rotation);
        *self = zoomed.get_block_coordinates();
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
/// These functions are used by [ApplyRotationTranslation] to rotate the I tetromino, which unlike the other tetrominos rotates around a block corner and not a block center.
impl Position {
    /// Get 2x zoomed coordinates for the block center.
    fn get_block_center_coordinates(&self) -> Self {
        Self::new(2 * self.x, 2 * self.y)
    }

    /// Get 2x zoomed coordinates for the block bottom right corner.
    fn get_block_bottom_right_coordinates(&self) -> Self {
        Self::new(2 * self.x + 1, 2 * self.y + 1)
    }

    /// Get block coordinates from 2x zoomed coordinates.
    fn get_block_coordinates(&self) -> Self {
        Self::new(self.x.div_euclid(2), self.y.div_euclid(2))
    }
}

#[cfg(test)]
mod tests {
    use super::super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
    use super::*;

    #[test]
    fn zoom_in_from_center_is_correct() {
        let pos = Position::new(5, -2);
        let expected = Position::new(10, -4);

        let actual = pos.get_block_center_coordinates();

        assert_eq!(actual, expected);
    }

    #[test]
    fn zoom_in_from_intersection_is_correct() {
        let pos = Position::new(-4, 3);
        let expected = Position::new(-7, 7);

        let actual = pos.get_block_bottom_right_coordinates();

        assert_eq!(actual, expected);
    }

    #[test]
    fn zoom_out_is_correct() {
        let pos = Position::new(-5, 5);
        let expected = Position::new(-3, 2);

        let actual = pos.get_block_coordinates();

        assert_eq!(actual, expected);
    }

    #[test]
    fn zoom_in_from_center_then_zoom_out_does_nothing() {
        for expected in [
            Position::new(0, 0),
            Position::new(-5, 2),
            Position::new(-4, 3),
        ] {
            let actual = expected
                .get_block_center_coordinates()
                .get_block_coordinates();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn zoom_in_from_intersection_then_zoom_out_does_nothing() {
        for expected in [
            Position::new(0, 0),
            Position::new(-5, 2),
            Position::new(-4, 3),
        ] {
            let actual = expected
                .get_block_bottom_right_coordinates()
                .get_block_coordinates();

            assert_eq!(actual, expected);
        }
    }

    const CLOCKWISE_TURN: RotationTranslation =
        RotationTranslation::rotation(RotationType::Clockwise, &Position::new(0, 0));

    #[test]
    fn translate_by_for_direction_does_nothing() {
        let expected = Direction::Up;
        let mut direction = expected;

        direction.translate_by(&CLOCKWISE_TURN);

        assert_eq!(direction, expected);
    }

    const ROTATION_TYPES: [RotationType; 4] = [
        RotationType::None,
        RotationType::Clockwise,
        RotationType::HalfTurn,
        RotationType::Counterclockwise,
    ];
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    #[test]
    fn turn_by_for_direction_is_correct() {
        for (rotation_type, expected) in ROTATION_TYPES.iter().zip(DIRECTIONS) {
            let mut instance = Direction::default();
            let rotation = RotationTranslation::rotation(*rotation_type, &Position::default());

            instance.turn_around_block_center(&rotation);

            assert_eq!(instance, expected);
        }
    }

    #[test]
    fn turn_by_with_offset_for_direction_is_identical_to_turn_by() {
        for rotation_type in ROTATION_TYPES {
            let mut instance = Direction::default();
            let mut expected = instance;
            let rotation = RotationTranslation::rotation(rotation_type, &Position::default());

            expected.turn_around_block_center(&rotation);
            instance.turn_around_block_corner(&rotation);

            assert_eq!(instance, expected);
        }
    }

    #[test]
    fn translate_by_for_position_is_correct() {
        let mut instance = Position::new(5, -2);
        let expected = Position::new(2, 2);
        let translation = Position::new(-3, 4);

        instance.translate_by(&RotationTranslation::translation(translation));

        assert_eq!(instance, expected);
    }

    const UNIT_POSITIONS: [Position; 4] = [RISE, RIGHT, FALL, LEFT];

    #[test]
    fn turn_by_for_position_is_correct() {
        for (rotation_type, expected) in ROTATION_TYPES.iter().zip(UNIT_POSITIONS) {
            let mut instance = UNIT_POSITIONS[0];
            let rotation = RotationTranslation::rotation(*rotation_type, &Position::default());

            instance.turn_around_block_center(&rotation);

            assert_eq!(instance, expected);
        }
    }

    const RISE_TURNED_WITH_OFFSET: [Position; 4] = [
        RISE,
        Position::new(2, 0),
        Position::new(1, 2),
        Position::new(-1, 1),
    ];

    #[test]
    fn turn_by_with_offset_for_position_is_correct() {
        for (rotation_type, expected) in ROTATION_TYPES.iter().zip(RISE_TURNED_WITH_OFFSET) {
            let mut instance = RISE;
            let rotation = RotationTranslation::rotation(*rotation_type, &Position::default());

            instance.turn_around_block_corner(&rotation);

            assert_eq!(instance, expected);
        }
    }

    #[test]
    fn move_by_for_position_is_correct() {
        let mut instance = Position::new(-2, 1);
        let center = Position::new(4, 3);
        let translation = Position::new(-3, 4);
        let rotation_translation =
            RotationTranslation::new(&translation, RotationType::Clockwise, &center);

        instance.move_by(&rotation_translation);

        assert_eq!(instance, Position::new(3, 1));
    }

    #[test]
    fn move_by_with_offset_for_position_is_correct() {
        let mut instance = Position::new(-2, 1);
        let center = Position::new(4, 3);
        let translation = Position::new(-3, 4);
        let rotation_translation =
            RotationTranslation::new(&translation, RotationType::Clockwise, &center);

        instance.move_by_with_offset(&rotation_translation);

        assert_eq!(instance, Position::new(4, 1));
    }
}
