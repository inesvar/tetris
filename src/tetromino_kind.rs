use crate::assets::TetrisColor;

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
            TetrominoKind::I => [5, 2, 3, 2, 4, 2, 5, 2, 6, 2],
            TetrominoKind::O => [5, 2, 4, 1, 4, 2, 5, 1, 5, 2],
            TetrominoKind::Z => [5, 2, 4, 1, 5, 1, 5, 2, 6, 2],
            TetrominoKind::J => [5, 2, 4, 1, 4, 2, 5, 2, 6, 2],
            TetrominoKind::L => [5, 2, 4, 2, 5, 2, 6, 2, 6, 1],
            TetrominoKind::T => [5, 2, 6, 2, 4, 2, 5, 2, 5, 1],
            TetrominoKind::S => [5, 2, 4, 2, 5, 2, 5, 1, 6, 1],
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
}
