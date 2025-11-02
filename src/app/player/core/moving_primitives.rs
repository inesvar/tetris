//! Define `trait` [ApplyRotationTranslation] for [super::spatial_primitives].
#![doc = mermaid!("moving_primitives_flowgraph.mmd")]
use super::{
    mermaid,
    rotation_translation::{RotationTranslation, RotationType},
    spatial_primitives::{Direction, Position},
};

/// Apply a [RotationTranslation] to a [super::spatial_primitives].
pub(super) trait ApplyRotationTranslation {
    /// Turn then translate as described by `movement`.
    fn move_by(&mut self, movement: &RotationTranslation) {
        self.turn_by(movement);
        self.translate_by(movement);
    }
    /// Like [ApplyRotationTranslation::move_by()], except `movement.rotation_center`
    /// is understood as the bottom right block corner, not the block center
    /// (the implementation relies on [ZoomInAndOut]).
    fn move_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.turn_by_with_offset(movement);
        self.translate_by(movement);
    }

    /// Translate by `movement.translation`.
    fn translate_by(&mut self, movement: &RotationTranslation);
    /// Turn around `movement.rotation_center` by `movement.rotation_type`.
    fn turn_by(&mut self, movement: &RotationTranslation);
    /// Like [ApplyRotationTranslation::turn_by()], except `movement.rotation_center`
    /// is understood as the bottom right block corner, not the block center
    /// (the implementation relies on [ZoomInAndOut]).
    fn turn_by_with_offset(&mut self, movement: &RotationTranslation);
}

/// Convert block coordinates to 2x zoomed coordinates and the other way around.
/// In 2x zoomed coordinates, block centers lie at even coordinates
/// while block corners lie at odd coordinates.
///
/// This trait is useful to rotate the I tetromino, which unlike the other
/// tetrominos rotates around a block corner and not a block center.
trait ZoomInAndOut {
    /// Map block coordinates to zoomed-in even coordinates (scales x2).
    fn get_block_center_coordinates(&self) -> Self;
    /// Map block coordinates to zoomed-in odd coordinates (scales x2, add 1).
    fn get_block_bottom_right_corner(&self) -> Self;
    /// Map zoomed-in coordinates to block coordinates (divide by 2, round towards -infinite).
    fn get_block_coordinates(&self) -> Self;
}

impl ApplyRotationTranslation for Position {
    fn translate_by(&mut self, movement: &RotationTranslation) {
        *self += movement.translation;
    }

    fn turn_by(&mut self, movement: &RotationTranslation) {
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

    fn turn_by_with_offset(&mut self, movement: &RotationTranslation) {
        let mut zoomed = self.get_block_center_coordinates();
        let mut zoomed_rotation = *movement;
        zoomed_rotation.rotation_center = zoomed_rotation
            .rotation_center
            .get_block_bottom_right_corner();
        zoomed.turn_by(&zoomed_rotation);
        *self = zoomed.get_block_coordinates();
    }
}

impl ApplyRotationTranslation for Direction {
    fn translate_by(&mut self, _movement: &RotationTranslation) {}

    fn turn_by(&mut self, movement: &RotationTranslation) {
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

    fn turn_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.turn_by(movement);
    }
}

impl ZoomInAndOut for Position {
    fn get_block_center_coordinates(&self) -> Self {
        Self::new(2 * self.x, 2 * self.y)
    }

    fn get_block_bottom_right_corner(&self) -> Self {
        Self::new(2 * self.x + 1, 2 * self.y + 1)
    }

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

        let actual = pos.get_block_bottom_right_corner();

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
                .get_block_bottom_right_corner()
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

            instance.turn_by(&rotation);

            assert_eq!(instance, expected);
        }
    }

    #[test]
    fn turn_by_with_offset_for_direction_is_identical_to_turn_by() {
        for rotation_type in ROTATION_TYPES {
            let mut instance = Direction::default();
            let mut expected = instance;
            let rotation = RotationTranslation::rotation(rotation_type, &Position::default());

            expected.turn_by(&rotation);
            instance.turn_by_with_offset(&rotation);

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

            instance.turn_by(&rotation);

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

            instance.turn_by_with_offset(&rotation);

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
