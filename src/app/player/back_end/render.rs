//! Define `trait` [Render] for [Block], [Tetromino] and [TetrisGrid].
use super::{tetris_block::Block, TetrisColor, TetrisGrid, Tetromino};
use crate::assets::Assets;
use crate::settings::{BLOCK_SIZE, GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS};
use graphics::draw_state::Blend;
use graphics::types::{Matrix2d, Rectangle, Scalar};
use graphics::{rectangle, DrawState, Image};
use opengl_graphics::GlGraphics;

/// Draw a tetris object at the given position.
pub(in crate::app::player) trait Render {
    fn render(
        &self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    );
}

impl Render for TetrisGrid {
    /// Draw a [TetrisGrid] and its contents at the given `grid_position`.
    fn render(
        &self,
        grid_position: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let empty_dims: Rectangle = [
            0.0,
            self.hidden_height(),
            self.total_width(),
            self.visible_height(),
        ];
        rectangle(GRID_BG_COLOR, empty_dims, grid_position, gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(empty_dims, draw_state, grid_position, gl);

        /* for (y, row) in self.matrix.iter().enumerate() {
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
        } */
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(tetris_color) = cell {
                    tetris_color.render(x, y, grid_position, draw_state, gl, assets);
                }
            }
        }
    }
}

impl Render for Tetromino {
    /// Draw a [Tetromino] (eventually a ghost) at the given `tetromino_position`.
    fn render(
        &self,
        grid_position: Matrix2d,
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
            self.blocks[i].render(grid_position, &draw_state, gl, assets);
        }
    }
}

impl Render for Block {
    /// Draw a [Block] using its position and the given `grid_position`.
    fn render(
        &self,
        grid_position: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let dims = rectangle::square(
            self.x() as Scalar * BLOCK_SIZE,
            self.y() as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            assets.texture_from_tetris_color(&self.color()),
            draw_state,
            grid_position,
            gl,
        );
    }
}

impl TetrisColor {
    /// Draw a [Block] using the specified [TetrisColor], `x` and `y` coordinates and `grid_position`.
    pub fn render(
        &self,
        x: usize,
        y: usize,
        grid_position: Matrix2d,
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
            grid_position,
            gl,
        );
    }
}
