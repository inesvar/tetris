//! Define `trait` [Render] for [Tetromino] and [TetrisGrid].
use super::core::{Position, TetrisGrid, Tetromino};
use crate::app::render_app::{Piston2dGraphicsArguments, Render};
use crate::assets::TetrisColor;
use crate::settings::{BLOCK_SIZE, GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS};
use graphics::types::{Rectangle, Scalar};
use graphics::{rectangle, Image};

/// In [super::render], helpers used to implement [crate::app::render_app::Render] for [TetrisGrid].
impl TetrisGrid {
    pub(in crate::app::player) fn total_width(&self) -> f64 {
        self.nb_columns() as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn visible_height(&self) -> f64 {
        self.nb_visible_rows() as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn hidden_height(&self) -> f64 {
        self.nb_hidden_rows() as f64 * BLOCK_SIZE
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

impl Render<Piston2dGraphicsArguments<'_, '_>> for TetrisGrid {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>) {
        let empty_dims: Rectangle = [
            0.0,
            self.hidden_height(),
            self.total_width(),
            self.visible_height(),
        ];
        rectangle(GRID_BG_COLOR, empty_dims, gl_ctx.transform, gl_ctx.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(empty_dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

        for position in self.positions() {
            if let Some(tetris_color) = self[&position] {
                position.render_tetris_block(&tetris_color, gl_ctx);
            }
        }
    }
}

impl Render<Piston2dGraphicsArguments<'_, '_>> for Tetromino {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>) {
        for block in self.blocks() {
            block.render_tetris_block(&self.color, gl_ctx);
        }
    }
}

impl Position {
    fn render_tetris_block(
        &self,
        tetris_color: &TetrisColor,
        gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>,
    ) {
        let dims = rectangle::square(
            self.x() as Scalar * BLOCK_SIZE,
            self.y() as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            gl_ctx.assets.texture_from_tetris_color(tetris_color),
            &gl_ctx.draw_state,
            gl_ctx.transform,
            gl_ctx.gl,
        );
    }
}
