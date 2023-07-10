use crate::assets::TetrisColor;
use crate::point::Point;
use crate::rotation::Rotation;
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

    pub fn get_initial_position(&self) -> [i8; 10] {
        match self {
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

    pub fn wall_kicks_translations(&self, rotation: i8, rotation_status: Rotation) -> [Point; 5] {
        match self {
            TetrominoKind::O => unreachable!(),
            TetrominoKind::I => match (rotation_status, rotation) {
                (Rotation::R0, 1) => [
                    Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, 0),
                    Point::new(1, 0) + Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, -1),
                    Point::new(1, 0) + Point::new(1, 2),
                ],
                (Rotation::R1, -1) => [
                    Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 0),
                    Point::new(-1, 0) + Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 1),
                    Point::new(-1, 0) + Point::new(-1, -2),
                ],
                (Rotation::R1, 1) => [
                    Point::new(0, 1),
                    Point::new(0, 1) + Point::new(-1, 0),
                    Point::new(0, 1) + Point::new(2, 0),
                    Point::new(0, 1) + Point::new(-1, 2),
                    Point::new(0, 1) + Point::new(2, -1),
                ],
                (Rotation::R2, -1) => [
                    Point::new(0, -1),
                    Point::new(0, -1) + Point::new(1, 0),
                    Point::new(0, -1) + Point::new(-2, 0),
                    Point::new(0, -1) + Point::new(1, -2),
                    Point::new(0, -1) + Point::new(-2, 1),
                ],
                (Rotation::R2, 1) => [
                    Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 0),
                    Point::new(-1, 0) + Point::new(-1, 0),
                    Point::new(-1, 0) + Point::new(2, 1),
                    Point::new(-1, 0) + Point::new(-1, -2),
                ],
                (Rotation::R3, -1) => [
                    Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, 0),
                    Point::new(1, 0) + Point::new(1, 0),
                    Point::new(1, 0) + Point::new(-2, -1),
                    Point::new(1, 0) + Point::new(1, 2),
                ],
                (Rotation::R3, 1) => [
                    Point::new(0, -1),
                    Point::new(0, -1) + Point::new(1, 0),
                    Point::new(0, -1) + Point::new(-2, 0),
                    Point::new(0, -1) + Point::new(1, -2),
                    Point::new(0, -1) + Point::new(-2, 1),
                ],
                (Rotation::R0, -1) => [
                    Point::new(0, 1),
                    Point::new(0, 1) + Point::new(-1, 0),
                    Point::new(0, 1) + Point::new(2, 0),
                    Point::new(0, 1) + Point::new(-1, 2),
                    Point::new(0, 1) + Point::new(2, -1),
                ],
                _ => unreachable!(),
            },
            _ => match (rotation_status, rotation) {
                (Rotation::R0, 1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ],
                (Rotation::R1, -1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ],
                (Rotation::R1, 1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, -1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                ],
                (Rotation::R2, -1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, 1),
                    Point::new(0, -2),
                    Point::new(-1, -2),
                ],
                (Rotation::R2, 1) => [
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, -2),
                    Point::new(1, -2),
                ],
                (Rotation::R3, -1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ],
                (Rotation::R3, 1) => [
                    Point::new(0, 0),
                    Point::new(-1, 0),
                    Point::new(-1, -1),
                    Point::new(0, 2),
                    Point::new(-1, 2),
                ],
                (Rotation::R0, -1) => [
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
