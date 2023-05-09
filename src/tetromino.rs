use crate::assets::Assets;
use crate::block::{Block, Collision};
use crate::point::{Point, Transformable};
use crate::rotation::Rotation;
use crate::tetromino_kind::TetrominoKind;
use crate::translate_rotate::TranslateRotate;
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
        self.blocks = self.check_possible(matrix, TranslateRotate::fall())?;
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
        match self.check_possible(matrix, TranslateRotate::left()) {
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
        match self.check_possible(matrix, TranslateRotate::right()) {
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
        let wall_kick_translations = TetrominoKind::wall_kicks_translations(&self.kind, 1, self.rotation_status);
        for i in 0..5 {
            match self.check_possible(matrix, TranslateRotate::new(wall_kick_translations[i], 1, &self.center)) {
                Err(()) => {
                    continue;
                }
                Ok(new_blocks) => {
                    self.blocks = new_blocks;
                    self.rotation_status.clockwise();
                    self.center.translate(&wall_kick_translations[i]);
                    return;
                }
            }
        }
    }

    pub fn turn_counterclockwise(&mut self, matrix: &Vec<Vec<Option<Block>>>) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kick_translations = TetrominoKind::wall_kicks_translations(&self.kind, -1, self.rotation_status);
        for i in 0..5 {
            match self.check_possible(matrix, TranslateRotate::new(wall_kick_translations[i], -1, &self.center)) {
                Err(()) => {
                    continue;
                }
                Ok(new_blocks) => {
                    self.blocks = new_blocks;
                    self.rotation_status.counterclockwise();
                    self.center.translate(&wall_kick_translations[i]);
                    return;
                }
            }
        }
    }

    pub fn check_possible(
        &self,
        matrix: &Vec<Vec<Option<Block>>>,
        movement: TranslateRotate
    ) -> Result<[Block; 4], ()> {
        let mut new_blocks = vec![];
        for i in 0..4 {
            new_blocks.push(self.blocks[i].move_to(matrix, &movement)?);
        }
        let blocks = [new_blocks[0], new_blocks[1], new_blocks[2], new_blocks[3]];
        Ok(blocks)
    }
}
