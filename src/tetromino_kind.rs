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


}
