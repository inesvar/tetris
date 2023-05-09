use crate::assets::TetrisColor;
use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(PartialEq, Copy, Clone)]
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
            TetrominoKind::I => [5, 1, 3, 1, 4, 1, 5, 1, 6, 1],
            TetrominoKind::O => [5, 1, 4, 0, 4, 1, 5, 0, 5, 1],
            TetrominoKind::Z => [5, 1, 4, 0, 5, 0, 5, 1, 6, 1],
            TetrominoKind::J => [5, 1, 4, 0, 4, 1, 5, 1, 6, 1],
            TetrominoKind::L => [5, 1, 4, 1, 5, 1, 6, 1, 6, 0],
            TetrominoKind::T => [5, 1, 6, 1, 4, 1, 5, 1, 5, 0],
            TetrominoKind::S => [5, 1, 4, 1, 5, 1, 5, 0, 6, 0],
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

    pub fn new_random_bag(size_of_bag: u32) -> Vec<TetrominoKind> {
        let mut tetromino_bag = vec!();
        let mut list = vec!();
        for i in 0..(size_of_bag - size_of_bag%7) {
            list.push(i % 7);
        }
        for _ in 0..(size_of_bag%7) {
            list.push(rand::random::<u32>()%7);
        }
        let mut rng = thread_rng();
        list.shuffle(&mut rng);
        println!("{:?}", list);
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
}
