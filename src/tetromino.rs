use crate::block::{Block, Collision};
use crate::point::{Point, Transformable};
use crate::tetromino_kind::TetrominoKind;
use graphics::types::Matrix2d;
use graphics::Context;
use graphics::draw_state::Blend;
use opengl_graphics::GlGraphics;

use crate::assets::Assets;
use crate::assets::TetrisColor;



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
    pub fn new(kind: TetrominoKind, matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        for i in 1..5 {
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


    pub fn new_random(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino>{
        match rand::random::<u8>() % 7 {
            0 => Tetromino::new(TetrominoKind::I, matrix),
            1 => Tetromino::new(TetrominoKind::O, matrix),
            2 => Tetromino::new(TetrominoKind::T, matrix),
            3 => Tetromino::new(TetrominoKind::S, matrix),
            4 => Tetromino::new(TetrominoKind::Z, matrix),
            5 => Tetromino::new(TetrominoKind::J, matrix),
            _ => Tetromino::new(TetrominoKind::L, matrix),
        }
    }

    pub fn reset_position(&mut self) {
        let positions = self.kind.get_initial_position();
        let color = self.kind.get_color();
        self.center = Point::new(positions[0], positions[1]);
        self.blocks = [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
            ];
    }

    fn translate(&mut self, x: i8, y: i8) {
        let translation = Point::new(x, y);
        self.center.translate(translation);
        for i in 0..4 {
            self.blocks[i].translate(translation);
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
