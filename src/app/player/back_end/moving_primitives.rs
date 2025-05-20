//! Define `trait` [ApplyRotationTranslation] for [Position] and [Block].
use super::{Block, Direction, Position, RotationTranslation, RotationType, TetrisGrid};
use delegate::delegate;

/// Apply a [RotationTranslation] to a spatial primitive.
pub(super) trait ApplyRotationTranslation {
    fn move_by(&mut self, movement: &RotationTranslation) {
        self.turn_by(movement);
        self.translate_by(movement);
    }
    fn move_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.turn_by_with_offset(movement);
        self.translate_by(movement);
    }
    fn translate_by(&mut self, movement: &RotationTranslation);
    fn turn_by(&mut self, movement: &RotationTranslation);
    fn turn_by_with_offset(&mut self, movement: &RotationTranslation);
}

// TODO create a trait (could be implemented for tetromino too) for this function.
impl Block {
    /// Move `self` by `movement` and return new position if it's available in `grid`.
    pub(super) fn can_be_moved(
        &self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
        rotation_center_is_on_block_center: bool,
    ) -> Result<Block, ()> {
        let mut block = *self;
        if rotation_center_is_on_block_center {
            block.move_by(movement);
        } else {
            block.move_by_with_offset(movement);
        }
        // Check if `copy` is inside `grid` and on an empty slot.
        if grid.is_block_available(&block) {
            Ok(block)
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

    fn turn_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.turn_by(movement);
    }
}

impl ApplyRotationTranslation for Block {
    delegate! {
        to self.position() {
            fn translate_by(&mut self, movement: &RotationTranslation);
            fn turn_by(&mut self, movement: &RotationTranslation);
            fn turn_by_with_offset(&mut self, movement: &RotationTranslation);
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

    fn turn_by_with_offset(&mut self, movement: &RotationTranslation) {
        self.zoom_in_from_center();
        let mut offset_movement = *movement;
        offset_movement.rotation_center.zoom_in_from_intersection();
        self.turn_by(&offset_movement);
        self.zoom_out();
    }
}

/// Scale the coordinate system to support different rotation centers.
///
/// Most tetromino rotations are performed around the center of a block.
/// However, the I tetromino rotates around the intersection between blocks,
/// which lies at the midpoint between standard coordinates.
///
/// Zooming in scale x2 and zooming out scale x0.5.
/// The bottom right block intersection is considered to have the same zoomed-out coordinates as the block center.
trait ZoomInAndOut {
    /// Maps block centers to even coordinates.
    fn zoom_in_from_center(&mut self);
    /// Maps block intersections to odd coordinates.
    fn zoom_in_from_intersection(&mut self);
    /// Maps zoomed-in coordinates to regular coordinates.
    fn zoom_out(&mut self);
}

impl ZoomInAndOut for Position {
    fn zoom_in_from_center(&mut self) {
        self.x *= 2;
        self.y *= 2;
    }

    fn zoom_in_from_intersection(&mut self) {
        self.x = 2 * self.x + 1;
        self.y = 2 * self.y + 1;
    }

    fn zoom_out(&mut self) {
        self.x /= 2;
        self.y /= 2;
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
        assert_eq!(point, center + vector.turned_clockwise());
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
        assert_eq!(point, center + vector.turned_clockwise() + translation);
    }
}
