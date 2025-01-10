use super::{Block, Position, RotationType, TetrisGrid, TranslationRotation};
use delegate::delegate;

pub(super) trait ApplyTranslationRotation {
    fn move_by(&mut self, movement: &TranslationRotation) {
        self.translate_by(movement);
        self.turn_by(movement);
    }
    fn translate_by(&mut self, movement: &TranslationRotation);
    fn turn_by(&mut self, movement: &TranslationRotation);
}

impl Block {
    pub(super) fn can_be_moved(
        &self,
        grid: &TetrisGrid,
        movement: &TranslationRotation,
    ) -> Result<Block, ()> {
        let mut copy = *self;
        copy.move_by(movement);
        // Check if `copy` is inside `grid` and on an empty slot
        if grid.is_block_available(&copy) {
            Ok(copy)
        } else {
            Err(())
        }
    }
}

impl ApplyTranslationRotation for Block {
    delegate! {
        to self.position() {
            fn translate_by(&mut self, movement: &TranslationRotation);
            fn turn_by(&mut self, movement: &TranslationRotation);
        }
    }
}

impl ApplyTranslationRotation for Position {
    fn translate_by(&mut self, movement: &TranslationRotation) {
        *self += &movement.translation;
    }

    fn turn_by(&mut self, movement: &TranslationRotation) {
        let center: &Position = &movement.rotation_center;
        match &movement.rotation_type {
            RotationType::Clockwise => {
                let vector = &*self - center;
                *self = center + vector.turned_clockwise();
            }
            RotationType::Counterclockwise => {
                let vector = &*self - center;
                *self = center + vector.turned_counterclockwise();
            }
            RotationType::HalfTurn => {
                let vector = &*self - center;
                *self = center + vector.neg();
            }
            RotationType::None => {}
        }
    }
}

#[test]
fn turns_around_arbitrary_center() {
    let center = Position::new(4, 3);
    let mut point = Position::new(-2, 1);
    let vector = &point - &center;
    let translation_rotation =
        TranslationRotation::new(&Position::default(), RotationType::Clockwise, &center);

    point.turn_by(&translation_rotation);
    assert_eq!(point, center.add(vector.turned_clockwise()));
}

#[test]
fn moves_by_translation_rotation() {
    let center = Position::new(4, 3);
    let translation = Position::new(-3, 4);
    let mut point = Position::new(-2, 1);
    let vector = &point - &center;
    let translation_rotation =
        TranslationRotation::new(&translation, RotationType::Clockwise, &center);

    point.move_by(&translation_rotation);
    assert_eq!(
        point,
        center.add(vector.turned_clockwise()).add(translation)
    );
}
