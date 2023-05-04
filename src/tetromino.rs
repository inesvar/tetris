use crate::block::{Block, Collision, NewBlock};
use crate::point::{Point, Transformable};
use graphics::types::Matrix2d;
use graphics::{rectangle, Context, Image};
use graphics::draw_state::Blend;
use opengl_graphics::GlGraphics;

use crate::assets::Assets;
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

#[derive(Clone, Copy)]
pub struct Tetromino {
    color: TetrisColor,
    kind: TetrominoKind,
    center: Point,
    blocks: [Block; 4],
    rotation_status: Rotation,
    is_ghost: bool,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

impl Rotation {
    fn clockwise(&mut self) {
        match self {
            Rotation::R0 => { *self = Rotation::R1; }
            Rotation::R1 => { *self = Rotation::R2; }
            Rotation::R2 => { *self = Rotation::R3; }
            Rotation::R3 => { *self = Rotation::R0; }
        }
    }

    fn counterclockwise(&mut self) {
        match self {
            Rotation::R0 => { *self = Rotation::R3; }
            Rotation::R1 => { *self = Rotation::R0; }
            Rotation::R2 => { *self = Rotation::R1; }
            Rotation::R3 => { *self = Rotation::R2; }
        }
    }
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
            is_ghost: false,
        }
    }

    pub fn new_T() -> Self {
        Tetromino::new(
            TetrisColor::PURPLE,
            TetrominoKind::T,
            &mut [5, 2, 1, 0, -1, 0, 0, 0, 0, -1],
        )
    }

    pub fn new_S() -> Self {
        Tetromino::new(
            TetrisColor::GREEN,
            TetrominoKind::S,
            &mut [5, 2, -1, 0, 0, 0, 0, -1, 1, -1],
        )
    }

    pub fn new_Z() -> Self {
        Tetromino::new(
            TetrisColor::RED,
            TetrominoKind::Z,
            &mut [5, 2, -1, -1, 0, -1, 0, 0, 1, 0],
        )
    }

    pub fn new_L() -> Self {
        Tetromino::new(
            TetrisColor::ORANGE,
            TetrominoKind::L,
            &mut [5, 2, -1, 0, 0, 0, 1, 0, 1, -1],
        )
    }

    pub fn new_J() -> Self {
        Tetromino::new(
            TetrisColor::BLUE,
            TetrominoKind::J,
            &mut [5, 2, -1, -1, -1, 0, 0, 0, 1, 0],
        )
    }

    pub fn new_O() -> Self {
        Tetromino::new(
            TetrisColor::YELLOW,
            TetrominoKind::O,
            &mut [5, 2, -1, -1, -1, 0, 0, -1, 0, 0],
        )
    }

    pub fn new_I() -> Self {
        Tetromino::new(
            TetrisColor::CYAN,
            TetrominoKind::I,
            &mut [5, 2, -2, 0, -1, 0, 0, 0, 1, 0],
        )
    }

    pub fn new_random() -> Self {
        match rand::random::<u8>() % 7 {
            0 => Tetromino::new_I(),
            1 => Tetromino::new_O(),
            2 => Tetromino::new_T(),
            3 => Tetromino::new_S(),
            4 => Tetromino::new_Z(),
            5 => Tetromino::new_J(),
            _ => Tetromino::new_L(),
        }
    }
}

/* RENDER METHOD */
impl Tetromino {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        for i in 0..4 {
            let draw_state = if self.is_ghost {
                ctx.draw_state.blend(Blend::Multiply)
            } else {
                ctx.draw_state
            };
            self.blocks[i].render(transform, &draw_state, gl, assets);
        }
    }
}

impl Tetromino {
    pub fn make_ghost_copy(&mut self) -> Tetromino {
        let mut ghost = self.clone();
        ghost.is_ghost = true;
        ghost
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

    pub fn hard_drop(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewTetromino {
        match self.fall(matrix) {
            NewTetromino::Error => { NewTetromino::Error }
            NewTetromino::Success => { self.hard_drop(matrix) }
        }
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
        self.rotation_status.clockwise();
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
        self.rotation_status.counterclockwise();
        NewTetromino::Success
    }
}
