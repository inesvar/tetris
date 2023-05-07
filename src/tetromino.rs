use crate::block::{Block, Collision};
use crate::point::{Point, Transformable};
use graphics::types::Matrix2d;
use graphics::Context;
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
    pub fn new(color: TetrisColor, kind: TetrominoKind, positions: &mut [i8], matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino> {
        for i in 1..5 {
            positions[2 * i] += positions[0];
            positions[2 * i + 1] += positions[1];
            if let Some(_) = matrix[positions[2 * i + 1] as usize][positions[2 * i] as usize] {
                return None;
            }
        }
        Some(Tetromino {
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
        })
    }

    pub fn new_T(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::PURPLE,
            TetrominoKind::T,
            &mut [5, 2, 1, 0, -1, 0, 0, 0, 0, -1],
            matrix,
        )
    }

    pub fn new_S(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::GREEN,
            TetrominoKind::S,
            &mut [5, 2, -1, 0, 0, 0, 0, -1, 1, -1],
            matrix,
        )
    }

    pub fn new_Z(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::RED,
            TetrominoKind::Z,
            &mut [5, 2, -1, -1, 0, -1, 0, 0, 1, 0],
            matrix,
        )
    }

    pub fn new_L(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::ORANGE,
            TetrominoKind::L,
            &mut [5, 2, -1, 0, 0, 0, 1, 0, 1, -1],
            matrix,
        )
    }

    pub fn new_J(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::BLUE,
            TetrominoKind::J,
            &mut [5, 2, -1, -1, -1, 0, 0, 0, 1, 0],
            matrix,
        )
    }

    pub fn new_O(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::YELLOW,
            TetrominoKind::O,
            &mut [5, 2, -1, -1, -1, 0, 0, -1, 0, 0],
            matrix,
        )
    }

    pub fn new_I(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        Tetromino::new(
            TetrisColor::CYAN,
            TetrominoKind::I,
            &mut [5, 2, -2, 0, -1, 0, 0, 0, 1, 0],
            matrix,
        )
    }

    pub fn new_random(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        match rand::random::<u8>() % 7 {
            0 => Tetromino::new_I(matrix),
            1 => Tetromino::new_O(matrix),
            2 => Tetromino::new_T(matrix),
            3 => Tetromino::new_S(matrix),
            4 => Tetromino::new_Z(matrix),
            5 => Tetromino::new_J(matrix),
            _ => Tetromino::new_L(matrix),
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

/* COLLISION METHODS */
impl Tetromino {
    pub fn reset_position(&mut self) {
        let new_center = Point::new(5, 2);
        let translation = new_center - self.center;

        for i in 0..4 {
            self.blocks[i].position.x += translation.x;
            self.blocks[i].position.y += translation.y;
        }

        self.center = new_center;
    }
    pub fn fall(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> Result<(),()> {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].fall(matrix) {
                Err(()) => {
                    return Err(());
                }
                Ok(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_down();
        Ok(())
    }

    pub fn hard_drop(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> () {
        match self.fall(matrix) {
            Err(()) => { return; }
            Ok(()) => { self.hard_drop(matrix) }
        }
    }

    pub fn left(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> () {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].left(matrix) {
                Err(()) => {
                    return;
                }
                Ok(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_left();
    }

    pub fn right(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> () {
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].right(matrix) {
                Err(()) => {
                    return;
                }
                Ok(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.center.go_right();
    }

    pub fn turn_clockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> () {
        if self.kind == TetrominoKind::O {
            return;
        };
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].turn_clockwise(&self.center, matrix) {
                Err(()) => {
                    return;
                }
                Ok(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.rotation_status.clockwise();
    }

    pub fn turn_counterclockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> () {
        if self.kind == TetrominoKind::O {
            return;
        };
        let mut new_blocks = vec![];
        for i in 0..4 {
            match self.blocks[i].turn_counterclockwise(&self.center, matrix) {
                Err(()) => {
                    return;
                }
                Ok(block) => {
                    new_blocks.push(block);
                }
            }
        }
        self.blocks.copy_from_slice(&new_blocks[0..4]);
        self.rotation_status.counterclockwise();
    }
}
