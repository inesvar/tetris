use crate::assets::TetrisColor;
use super::point::Point;
use super::rotation_state::RotationState;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoKind {
    pub fn get(&self) -> String {
        match self {
            TetrominoKind::I => "I".to_owned(),
            TetrominoKind::O => "O".to_owned(),
            TetrominoKind::Z => "Z".to_owned(),
            TetrominoKind::J => "J".to_owned(),
            TetrominoKind::L => "L".to_owned(),
            TetrominoKind::T => "T".to_owned(),
            TetrominoKind::S => "S".to_owned(),
        }
    }

    // returns the initial position of the center and the blocks of a tetromino
    pub fn get_initial_position(&self) -> [i8; 10] {
        // cf https://tetris.fandom.com/wiki/SRS#Spawn_Orientation_and_Location
        match self {
            // in order : center_x, center_y, first_block_x, first_block_y, second_block_x, second_block_y...
            TetrominoKind::I => [4, 1, 3, 1, 4, 1, 5, 1, 6, 1],
            TetrominoKind::O => [5, 1, 4, 0, 4, 1, 5, 0, 5, 1],
            TetrominoKind::Z => [4, 1, 3, 0, 4, 0, 4, 1, 5, 1],
            TetrominoKind::J => [4, 1, 3, 0, 3, 1, 4, 1, 5, 1],
            TetrominoKind::L => [4, 1, 3, 1, 4, 1, 5, 1, 5, 0],
            TetrominoKind::T => [4, 1, 5, 1, 3, 1, 4, 1, 4, 0],
            TetrominoKind::S => [4, 1, 3, 1, 4, 1, 4, 0, 5, 0],
        }
    }

    pub fn get_color(&self) -> TetrisColor {
        match self {
            TetrominoKind::I => TetrisColor::Cyan,
            TetrominoKind::O => TetrisColor::Yellow,
            TetrominoKind::Z => TetrisColor::Red,
            TetrominoKind::J => TetrisColor::Blue,
            TetrominoKind::L => TetrisColor::Orange,
            TetrominoKind::T => TetrisColor::Purple,
            TetrominoKind::S => TetrisColor::Green,
        }
    }

    // returns an array of the 5 SRS wall kick translations
    pub fn wall_kicks_translations(&self, rotation: i8, rotation_status: RotationState) -> [Point; 5] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match self {
            // since the O piece doesn't even rotate
            TetrominoKind::O => unreachable!(), 
            // the additional calculation for the I piece is to compensate for the fact
            // that its rotational center is actually between two blocks 
            // and not on a block like in the code
            // (the true position of the initial center is (4.5, 1) and not (4, 1) see line 32.)
            // (its due to the fact that the I piece doesn't have a 3x3 bounding box like the other rotating pieces)
            TetrominoKind::I => match (rotation_status, rotation) {
                (RotationState::R0, 1) => [
                    Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, 0),
                    Point::new(1, 0) + Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, -1),
                    Point::new(1, 0) + Point::new(1, 2),
                ],
                (RotationState::R1, -1) => [
                    Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 0),
                    Point::new(-1, 0) + Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 1),
                    Point::new(-1, 0) + Point::new(-1, -2),
                ],
                (RotationState::R1, 1) => [
                    Point::new(0, 1),
                    Point::new(0, 1) + Point::new(-1, 0),
                    Point::new(0, 1) + Point::new(2, 0),
                    Point::new(0, 1) + Point::new(-1, 2),
                    Point::new(0, 1) + Point::new(2, -1),
                ],
                (RotationState::R2, -1) => [
                    Point::new(0, -1),
                    Point::new(0, -1) + Point::new(1, 0),
                    Point::new(0, -1) + Point::new(-2, 0),
                    Point::new(0, -1) + Point::new(1, -2),
                    Point::new(0, -1) + Point::new(-2, 1),
                ],
                (RotationState::R2, 1) => [
                    Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 0),
                    Point::new(-1, 0) + Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 1),
                    Point::new(-1, 0) + Point::new(-1, -2),
                ],
                (RotationState::R3, -1) => [
                    Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, 0),
                    Point::new(1, 0) + Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, -1),
                    Point::new(1, 0) + Point::new(1, 2),
                ],
                (RotationState::R3, 1) => [
                    Point::new(0, -1),
                    Point::new(0, -1) + Point::new(1, 0),
                    Point::new(0, -1) + Point::new(-2, 0),
                    Point::new(0, -1) + Point::new(1, -2),
                    Point::new(0, -1) + Point::new(-2, 1),
                ],
                (RotationState::R0, -1) => [
                    Point::new(0, 1),
                    Point::new(0, 1) + Point::new(-1, 0),
                    Point::new(0, 1) + Point::new(2, 0),
                    Point::new(0, 1) + Point::new(-1, 2),
                    Point::new(0, 1) + Point::new(2, -1),
                ],
                _ => unreachable!(),
            },
            _ => match (rotation_status, rotation) {
                (RotationState::R0, 1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ],
                (RotationState::R1, -1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ],
                (RotationState::R1, 1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ],
                (RotationState::R2, -1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ],
                (RotationState::R2, 1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                ],
                (RotationState::R3, -1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ],
                (RotationState::R3, 1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ],
                (RotationState::R0, -1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                ],
                _ => unreachable!(),
            },
        }
    }
}
