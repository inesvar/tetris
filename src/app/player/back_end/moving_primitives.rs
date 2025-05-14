//! Define `trait` [ApplyRotationTranslation] for [Position] and [Block].
use super::{Block, Direction, Position, RotationTranslation, RotationType, TetrisGrid};
use delegate::delegate;

/// Apply a [RotationTranslation] to a spatial primitive.
pub(super) trait ApplyRotationTranslation {
    fn move_by(&mut self, movement: &RotationTranslation) {
        self.turn_by(movement);
        self.translate_by(movement);
    }
    fn translate_by(&mut self, movement: &RotationTranslation);
    fn turn_by(&mut self, movement: &RotationTranslation);
}

// TODO create a trait (could be implemented for tetromino too) for this function.
impl Block {
    /// Move `self` by `movement` and return new position if it's available in `grid`.
    pub(super) fn can_be_moved(
        &self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
    ) -> Result<Block, ()> {
        let mut copy = *self;
        copy.move_by(movement);
        // Check if `copy` is inside `grid` and on an empty slot.
        if grid.is_block_available(&copy) {
            Ok(copy)
        } else {
            Err(())
        }
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
}

impl ApplyRotationTranslation for Block {
    delegate! {
        to self.position() {
            fn translate_by(&mut self, movement: &RotationTranslation);
            fn turn_by(&mut self, movement: &RotationTranslation);
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::{ApplyRotationTranslation, Position, RotationTranslation, RotationType};

    #[test]
    fn rotation_is_correct() {
        let center = Position::new(4, 3);
        let mut point = Position::new(-2, 1);
        let vector = point - center;
        let rotation_translation =
            RotationTranslation::new(&Position::default(), RotationType::Clockwise, &center);

        point.turn_by(&rotation_translation);
        assert_eq!(point, center.add(vector.turned_clockwise()));
    }

    #[test]
    fn rotation_translation_is_correct() {
        let center = Position::new(4, 3);
        let translation = Position::new(-3, 4);
        let mut point = Position::new(-2, 1);
        let vector = point - center;
        let rotation_translation =
            RotationTranslation::new(&translation, RotationType::Clockwise, &center);

        point.move_by(&rotation_translation);
        assert_eq!(
            point,
            center.add(vector.turned_clockwise()).add(translation)
        );
    }
}
