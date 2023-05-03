use crate::block::{Block, Collision, NewBlock};
use crate::point::{Point, Transformable};
use graphics::types::Matrix2d;
use graphics::{rectangle, Context, Image};
use opengl_graphics::GlGraphics;

use crate::assets::Assets;
use crate::assets::TetrisColor;

#[derive(PartialEq)]
pub enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub struct Tetromino {
    color: TetrisColor,
    kind: TetrominoKind,
    center: Point,
    blocks: [Block; 4],
    rotation_status: Rotation,
}

pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

impl Tetromino {
    pub fn new(color: TetrisColor, kind: TetrominoKind, positions: &mut [i8]) -> Self {
        for i in 1..5 {
            positions[2 * i] += positions[0];
            positions[2 * i + 1] += positions[1];
        }
        Tetromino {
            color,
            kind,
            center: Point::new(positions[0], positions[1]),
            blocks: [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
            ],
            rotation_status: Rotation::R0,
        }
    }

    pub fn new_T() -> Self {
        Tetromino::new(
            TetrisColor::PURPLE,
            TetrominoKind::T,
            &mut [5, 2, 1, 0, -1, 0, 0, 0, 0, -1]
        )
    }

    pub fn new_S() -> Self {
        Tetromino::new(
            TetrisColor::GREEN,
            TetrominoKind::S,
            &mut [5, 2, -1, 0, 0, 0, 0, -1, 1, -1]
        )
    }

    pub fn new_Z() -> Self {
        Tetromino::new(
            TetrisColor::RED,
            TetrominoKind::Z,
            &mut [5, 2, -1, -1, 0, -1, 0, 0, 1, 0]
        )
    }

    pub fn new_L() -> Self {
        Tetromino::new(
            TetrisColor::ORANGE,
            TetrominoKind::L,
            &mut [5, 2, -1, 0, 0, 0, 1, 0, 1, -1]
        )
    }

    pub fn new_J() -> Self {
        Tetromino::new(
            TetrisColor::BLUE,
            TetrominoKind::J,
            &mut [5, 2, -1, -1, -1, 0, 0, 0, 1, 0]
        )
    }

    pub fn new_O() -> Self {
        Tetromino::new(
            TetrisColor::YELLOW,
            TetrominoKind::O,
            &mut [5, 2, -1, -1, -1, 0, 0, -1, 0, 0]
        )
    }

    pub fn new_I() -> Self {
        Tetromino::new(
            TetrisColor::CYAN,
            TetrominoKind::I,
            &mut [5, 2, -2, 0, -1, 0, 0, 0, 1, 0]
        )
    }
}

/* RENDER METHOD */
impl Tetromino {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        for i in 0..4 {
            self.blocks[i].render(transform, &ctx.draw_state, gl, assets);
        }
    }
}

impl Tetromino {
    pub fn split(&mut self) -> [Block; 4] {
        self.blocks
    }
}

pub enum NewTetromino {
    Error,
    Success,
}


/* COLLISION METHODS */
impl Tetromino {
    pub fn fall(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].fall(matrix) {
                NewBlock::Error => {
                    return NewTetromino::Error;
                }
                NewBlock::Success(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_down();
        NewTetromino::Success
    }

    pub fn left(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].left(matrix) {
                NewBlock::Error => {
                    return NewTetromino::Error;
                }
                NewBlock::Success(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_left();
        NewTetromino::Success
    }

    pub fn right(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].right(matrix) {
                NewBlock::Error => {
                    return NewTetromino::Error;
                }
                NewBlock::Success(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_right();
        NewTetromino::Success
    }

    pub fn turn_clockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        if self.kind == TetrominoKind::O {
            return NewTetromino::Success;
        };
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].turn_clockwise(&self.center, matrix) {
                NewBlock::Error => {
                    return NewTetromino::Error;
                }
                NewBlock::Success(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        NewTetromino::Success
    }

    pub fn turn_counterclockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        if self.kind == TetrominoKind::O {
            return NewTetromino::Success;
        };
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].turn_counterclockwise(&self.center, matrix) {
                NewBlock::Error => {
                    return NewTetromino::Error;
                }
                NewBlock::Success(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        NewTetromino::Success
    }
}
