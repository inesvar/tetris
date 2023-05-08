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
            TetrominoKind::I => return [5, 2, 3, 2, 4, 2, 5, 2, 6, 2],
            TetrominoKind::O => return [5, 2, 4, 1, 4, 2, 5, 1, 5, 2],
            TetrominoKind::Z => return [5, 2, 4, 1, 5, 1, 5, 2, 6, 2],
            TetrominoKind::J => return [5, 2, 4, 1, 4, 2, 5, 2, 6, 2],
            TetrominoKind::L => return [5, 2, 4, 2, 5, 2, 6, 2, 6, 1],
            TetrominoKind::T => return [5, 2, 6, 2, 4, 2, 5, 2, 5, 1],
            TetrominoKind::S => return [5, 2, 4, 2, 5, 2, 5, 1, 6, 1],
        }
    }

    pub fn get_color(&self) -> TetrisColor {
        match self {
            TetrominoKind::I => return TetrisColor::CYAN,
            TetrominoKind::O => return TetrisColor::YELLOW,
            TetrominoKind::Z => return TetrisColor::RED,
            TetrominoKind::J => return TetrisColor::BLUE,
            TetrominoKind::L => return TetrisColor::ORANGE,
            TetrominoKind::T => return TetrisColor::PURPLE,
            TetrominoKind::S => return TetrisColor::GREEN,
        }
    }

    
}