use crate::assets::TetrisColor;
use crate::point::Point;
use crate::rotation::Rotation;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

#[derive(PartialEq, Copy, Clone, Serialize)]
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
            TetrominoKind::I => TetrisColor::CYAN,
            TetrominoKind::O => TetrisColor::YELLOW,
            TetrominoKind::Z => TetrisColor::RED,
            TetrominoKind::J => TetrisColor::BLUE,
            TetrominoKind::L => TetrisColor::ORANGE,
            TetrominoKind::T => TetrisColor::PURPLE,
            TetrominoKind::S => TetrisColor::GREEN,
        }
    }

    pub fn new_random_bag(mut size_of_bag: u32) -> Vec<TetrominoKind> {
        if size_of_bag == 0 {
            size_of_bag = 1;
        }
        let mut tetromino_bag = vec![];
        let mut list = vec![];
        for i in 0..(size_of_bag + (7 - size_of_bag % 7) % 7) {
            // the smaller multiple of seven yet higher than size_of_bag
            list.push(i % 7);
        }
        let mut rng = thread_rng();
        list.shuffle(&mut rng);
        for _ in 0..(size_of_bag % 7) {
            list.pop();
        }
        for i in 0..size_of_bag {
            match list[i as usize] {
                0 => tetromino_bag.push(TetrominoKind::I),
                1 => tetromino_bag.push(TetrominoKind::O),
                2 => tetromino_bag.push(TetrominoKind::T),
                3 => tetromino_bag.push(TetrominoKind::S),
                4 => tetromino_bag.push(TetrominoKind::Z),
                5 => tetromino_bag.push(TetrominoKind::J),
                _ => tetromino_bag.push(TetrominoKind::L),
            }
        }
        tetromino_bag
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
