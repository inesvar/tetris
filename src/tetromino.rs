use crate::assets::Assets;
use crate::block::{Block, Collision, TranslateRotate};
use crate::point::{Point, Transformable};
use crate::rotation::Rotation;
use crate::tetromino_kind::TetrominoKind;
use graphics::draw_state::Blend;
use graphics::types::Matrix2d;
use graphics::Context;
use opengl_graphics::GlGraphics;

#[derive(Clone, Copy)]
pub struct Tetromino {
    kind: TetrominoKind,
    center: Point,
    blocks: [Block; 4],
    rotation_status: Rotation,
    is_ghost: bool,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        for i in 1..5 {
            if matrix[positions[2 * i + 1] as usize][positions[2 * i] as usize].is_some() {
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

    pub fn new_random(matrix: &Vec<Vec<Option<Block>>>) -> Option<Tetromino> {
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
        let mut ghost = *self;
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
    pub fn fall(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> Result<(), ()> {
        self.blocks = self.check_possible(matrix, 0, 1, 0)?;
        self.center.go_down();
        Ok(())
    }

    pub fn hard_drop(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        match self.fall(matrix) {
            Err(()) => {}
            Ok(()) => self.hard_drop(matrix),
        }
    }

    pub fn left(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        match self.check_possible(matrix, -1, 0, 0) {
            Err(()) => {
                return;
            }
            Ok(new_blocks) => {
                self.blocks = new_blocks;
                self.center.go_left();
            }
        }
    }

    pub fn right(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        match self.check_possible(matrix, 1, 0, 0) {
            Err(()) => {
                return;
            }
            Ok(new_blocks) => {
                self.blocks = new_blocks;
                self.center.go_right();
            }
        }
    }

    pub fn turn_clockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        if self.kind == TetrominoKind::O {
            return;
        };
        match self.check_possible(matrix, 0, 0, 1) {
            Err(()) => {
                return;
            }
            Ok(new_blocks) => {
                self.blocks = new_blocks;
                self.rotation_status.clockwise();
            }
        }
    }

    pub fn turn_counterclockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        if self.kind == TetrominoKind::O {
            return;
        };
        match self.check_possible(matrix, 0, 0, -1) {
            Err(()) => {
                return;
            }
            Ok(new_blocks) => {
                self.blocks = new_blocks;
                self.rotation_status.counterclockwise();
            }
        }
    }

    pub fn check_possible(
        &self,
        matrix: &Vec<Vec<Option<Block>>>,
        x: i8,
        y: i8,
        rotation: i8,
    ) -> Result<[Block; 4], ()> {
        let translation = Point::new(x, y);
        let movement = TranslateRotate::new(translation, rotation, &self.center);
        let mut new_blocks = vec![];
        for i in 0..4 {
            new_blocks.push(self.blocks[i].move_to(matrix, &movement)?);
        }
        let blocks = [new_blocks[0], new_blocks[1], new_blocks[2], new_blocks[3]];
        Ok(blocks)
    }
}
