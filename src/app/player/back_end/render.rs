//! Defines the render functions of types [Block](super::block::Block::render()), [Tetromino](super::Tetromino::render()) and [TetrisGrid](super::TetrisGrid::render()).
use super::{block::Block, TetrisGrid, Tetromino};
use crate::assets::Assets;
use crate::assets::TetrisColor;
use crate::settings::{BLOCK_SIZE, GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS};
use graphics::types::{Matrix2d, Rectangle, Scalar};
use graphics::{draw_state::Blend, Transformed};
use graphics::{rectangle, DrawState, Image};
use opengl_graphics::GlGraphics;

impl TetrisGrid {
    /// Render the TetrisGrid and its contents.
    pub fn render(
        &mut self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        self.transform = transform.trans(self.x, self.y);

        let empty_dims: Rectangle = [
            0.0,
            self.total_height - self.visible_height,
            self.visible_width,
            self.visible_height,
        ];
        rectangle(GRID_BG_COLOR, empty_dims, self.transform, gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(empty_dims, draw_state, self.transform, gl);

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, _cell) in row.iter().enumerate() {
                if y > 1 {
                    let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
                    let outline_dims = rectangle::square(
                        x as Scalar * BLOCK_SIZE, // + GRID_THICKNESS as Scalar / 20.0,
                        y as Scalar * BLOCK_SIZE, // + GRID_THICKNESS as Scalar / 20.0,
                        BLOCK_SIZE,
                    );
                    outline_rect.draw(outline_dims, draw_state, self.transform, gl);
                }
            }
        }
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Some(tetris_color) => {
                        tetris_color.render(x, y, self.transform, draw_state, gl, assets)
                    }
                    None => {}
                }
            }
        }
    }
}

impl Tetromino {
    /// Render the Tetromino, and eventually the ghost Tetromino.
    pub fn render(
        &self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let draw_state = if self.is_ghost {
            draw_state.blend(Blend::Multiply)
        } else {
            *draw_state
        };
        for i in 0..4 {
            self.blocks[i].render(transform, &draw_state, gl, assets);
        }
    }
}

// TODO : make all the render signature the same so it can become a trait
impl Block {
    /// Render the Block using the texture from assets.
    pub fn render(
        &self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let dims = rectangle::square(
            self.position.x as Scalar * BLOCK_SIZE,
            self.position.y as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            assets.texture_from_tetris_color(&self.color),
            draw_state,
            transform,
            gl,
        );
    }
}

impl TetrisColor {
    /// Render the Block using the texture from assets.
    pub fn render(
        &self,
        x: usize,
        y: usize,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let dims = rectangle::square(
            x as Scalar * BLOCK_SIZE,
            y as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            assets.texture_from_tetris_color(self),
            draw_state,
            transform,
            gl,
        );
    }
}
