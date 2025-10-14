//! Define `trait` [ApplyRotationTranslation] for [super::spatial_primitives].
#![doc = mermaid!("moving_primitives_flowgraph.mmd")]
use super::{mermaid, Block, Direction, Position, RotationTranslation, RotationType, TetrisGrid, BackendError};

impl Block {
    /// Try to apply a [RotationTranslation] in a [TetrisGrid].
    /// Move `self` by `movement` and return whether it's valid on `grid`.
    pub(super) fn try_move(
        &mut self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
        is_rotation_center_on_block_center: bool,
    ) -> Result<(), BackendError> {
        if is_rotation_center_on_block_center {
            self.center.move_by(movement);
        } else {
            self.center.move_by_with_offset(movement);
        }
        // Check if `copy` is inside `grid` and on an empty slot.
        grid.is_block_available(self)
    }
}

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

/// Scale coordinates to support rotations around block intersections,
/// not just around block centers.
///
/// While most tetromino rotations are performed around a block center,
/// the I tetromino rotates around a block intersection,
/// which lies at the midpoint between standard coordinates.
///
/// This trait provides zooming in functions that scale x2 and a zooming out function that scales x0.5.
/// A block center has the same 'regular' (or zoomed-out) coordinates as the bottom right block intersection.
trait ZoomInAndOut {
    /// Map block centers to even coordinates.
    fn zoom_in_from_center(&mut self);
    /// Map block intersections to odd coordinates.
    fn zoom_in_from_intersection(&mut self);
    /// Map zoomed-in coordinates to regular coordinates.
    fn zoom_out(&mut self);
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
    use super::*;

    // TODO: clean unit tests

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
