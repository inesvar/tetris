//! Defines `struct` [Block] and `struct` [Position].
use super::{
    translation_rotation::Rotation, ApplyTranslationRotation, TetrisGrid, TranslationRotation,
};
use crate::assets::TetrisColor;
use delegate::delegate;
use serde::{Deserialize, Serialize};

/// Block in a discrete grid, serializable.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(super) struct Block {
    position: Position,
    color: TetrisColor,
}

/// Position on a discrete grid, serializable.
#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub(super) struct Position {
    /// horizontal coordinate, from left to right
    x: i8,
    /// vertical coordinate, *from top to bottom*
    y: i8,
}

// TODO move to module moving_primitives
impl ApplyTranslationRotation for Block {
    delegate! {
        to self.position {
            fn translate_by(&mut self, movement: &TranslationRotation);
            fn turn_by(&mut self, movement: &TranslationRotation);
        }
    }
}

// TODO move to module moving_primitives
impl ApplyTranslationRotation for Position {
    fn translate_by(&mut self, movement: &TranslationRotation) {
        *self += &movement.translation;
    }

    fn turn_by(&mut self, movement: &TranslationRotation) {
        match &movement.rotation {
            Rotation::Clockwise(center) => {
                let vector = &*self - center;
                *self = center + vector.turned_clockwise();
            }
            Rotation::Counterclockwise(center) => {
                let vector = &*self - center;
                *self = center + vector.turned_counterclockwise();
            }
            Rotation::NoRotation => {}
        }
    }
}

impl Block {
    pub(super) fn new(color: TetrisColor, x: i8, y: i8) -> Self {
        Block {
            position: Position::new(x, y),
            color,
        }
    }

    pub(super) fn x(&self) -> i8 {
        self.position.x
    }

    pub(super) fn y(&self) -> i8 {
        self.position.y
    }

    pub(super) fn color(&self) -> TetrisColor {
        self.color
    }

    // TODO move to module moving_primitives
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

impl Position {
    pub(super) const fn new(x: i8, y: i8) -> Self {
        Position { x, y }
    }

    // TODO create a const trait Arithmetic when it will be possible
    #[allow(dead_code)]
    pub(super) const fn neg(self) -> Position {
        Position::new(-self.x, -self.y)
    }

    pub(super) const fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }

    pub(super) const fn turned_clockwise(&self) -> Self {
        Position::new(self.y, -self.x)
    }

    pub(super) const fn turned_counterclockwise(&self) -> Self {
        Position::new(-self.y, self.x)
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Position::default(),
            color: TetrisColor::Yellow,
        }
    }
}

impl std::ops::Add<Position> for &Position {
    type Output = Position;
    fn add(self, other: Position) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub<Position> for &Position {
    type Output = Position;
    fn sub(self, other: Position) -> Self::Output {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add for &Position {
    type Output = Position;
    fn add(self, other: &Position) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for &Position {
    type Output = Position;
    fn sub(self, other: &Position) -> Self::Output {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign<&Position> for Position {
    fn add_assign(&mut self, other: &Position) {
        *self = &*self + other;
    }
}
