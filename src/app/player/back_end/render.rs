//! Define `trait` [Render] for [Block], [Tetromino] and [TetrisGrid].
use super::{spatial_primitives::Position, TetrisColor, TetrisGrid, Tetromino, UseTetrisGrid};
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

impl TetrisGrid {
    pub(in crate::app::player) fn total_width(&self) -> f64 {
        self.nb_columns as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn visible_height(&self) -> f64 {
        (self.nb_rows - self.nb_hidden_rows) as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn hidden_height(&self) -> f64 {
        self.nb_hidden_rows as f64 * BLOCK_SIZE
    }

    fn draw_on_empty_grid(&mut self, blocks: &[Position], tetris_color: TetrisColor) {
        self.reset();

        for block in blocks {
            self[block] = Some(tetris_color);
        }
    }

    const ONE: [Position; 9] = [
        Position::new(5, 9),
        Position::new(4, 10),
        Position::new(5, 10),
        Position::new(5, 11),
        Position::new(5, 12),
        Position::new(3, 13),
        Position::new(4, 13),
        Position::new(5, 13),
        Position::new(6, 13),
    ];

    const TWO: [Position; 10] = [
        Position::new(4, 9),
        Position::new(5, 9),
        Position::new(3, 10),
        Position::new(6, 10),
        Position::new(5, 11),
        Position::new(4, 12),
        Position::new(3, 13),
        Position::new(4, 13),
        Position::new(5, 13),
        Position::new(6, 13),
    ];

    const THREE: [Position; 9] = [
        Position::new(4, 9),
        Position::new(5, 9),
        Position::new(3, 10),
        Position::new(6, 10),
        Position::new(5, 11),
        Position::new(3, 12),
        Position::new(6, 12),
        Position::new(4, 13),
        Position::new(5, 13),
    ];

    /// Draw a 1 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn one(&mut self, tetris_color: TetrisColor) {
        self.draw_on_empty_grid(&Self::ONE, tetris_color);
    }

    /// Draw a 2 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn two(&mut self, tetris_color: TetrisColor) {
        self.draw_on_empty_grid(&Self::TWO, tetris_color);
    }

    /// Draw a 3 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn three(&mut self, tetris_color: TetrisColor) {
        self.draw_on_empty_grid(&Self::THREE, tetris_color);
    }
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
    /// Draw a [Tetromino] (eventually a ghost).
    fn render(
        &self,
        grid_position: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        for block in self.blocks {
            self.color.render(
                block.x as usize,
                block.y as usize,
                grid_position,
                draw_state,
                gl,
                assets,
            );
        }
    }
}

impl Tetromino {
    /// Draw a semi-transparent [Tetromino].
    pub(in crate::app::player) fn render_ghost(
        &self,
        grid_position: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let draw_state = draw_state.blend(Blend::Multiply);
        for block in self.blocks {
            self.color.render(
                block.x as usize,
                block.y as usize,
                grid_position,
                &draw_state,
                gl,
                assets,
            );
        }
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
