//! Define `trait` [Render] for [LocalPlayer], [PlayerScreen], [Tetromino], [TetrisGrid].
use super::core::{Position, TetrisGrid, Tetromino, NB_VISIBLE_BUFFER_ROWS};
use super::TetrisColor;
use crate::app::player::LocalPlayer;
use crate::app::render_app::{Piston2dGraphicsArguments, Render};
use crate::app::ui::text::Text;
use crate::app::PlayerScreen;
use crate::settings::{
    BLOCK_SIZE, DEFAULT_FONT_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, DEFAULT_SCORE_TEXT_Y,
    GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_NEXT_TETROMINO, TETROMINO_MAX_HEIGHT,
    TETROMINO_MAX_WIDTH, TEXT_COLOR,
};
use graphics::types::{Rectangle, Scalar};
use graphics::{rectangle, Image, Transformed};

impl Render<Piston2dGraphicsArguments<'_, '_>> for LocalPlayer {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        self.player_screen.render(gl_ctx);
    }
}

impl Render<Piston2dGraphicsArguments<'_, '_>> for PlayerScreen {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        let score_text = Text::new(
            format!("Score: {}", self.score).as_str(),
            DEFAULT_FONT_SIZE,
            // the score is centered under the hold piece rectangle
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
            DEFAULT_SCORE_TEXT_Y,
            TEXT_COLOR,
        );
        score_text.render(gl_ctx);

        let old_transform = gl_ctx.transform;
        let grid_transform = gl_ctx.transform.trans(DEFAULT_GRID_X, DEFAULT_GRID_Y);
        gl_ctx.transform = grid_transform;

        self.grid.render(gl_ctx);

        if let Some(ghost) = self.ghost_tetromino {
            let old_draw_state = gl_ctx.draw_state;
            gl_ctx.draw_state = gl_ctx
                .draw_state
                .blend(graphics::draw_state::Blend::Multiply);
            ghost.render(gl_ctx);
            gl_ctx.draw_state = old_draw_state;
        }

        self.active_tetromino.render(gl_ctx);

        // drawing a border for the hold piece
        gl_ctx.transform = grid_transform.trans(
            -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
            self.grid.hidden_height(),
        );
        let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
        let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
        rectangle(GRID_BG_COLOR, dims, gl_ctx.transform, gl_ctx.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

        // drawing the hold piece
        if let Some(saved) = self.saved_tetromino {
            gl_ctx.transform = grid_transform.trans(
                -TETROMINO_MAX_WIDTH - 2.0 * BLOCK_SIZE,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            saved.render(gl_ctx);
        }

        // drawing a border for the fifo of next pieces
        gl_ctx.transform = grid_transform.trans(
            self.grid.total_width() + BLOCK_SIZE,
            self.grid.hidden_height(),
        );
        let width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let height = BLOCK_SIZE + (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * NB_NEXT_TETROMINO as f64;
        let dims: Rectangle = [0.0, 0.0, width, height];
        rectangle(GRID_BG_COLOR, dims, gl_ctx.transform, gl_ctx.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

        // drawing the next pieces
        for i in 0..NB_NEXT_TETROMINO {
            gl_ctx.transform = grid_transform.trans(
                self.grid.total_width() + 2.0 * BLOCK_SIZE,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = self.fifo_next_tetromino.get(i) {
                tetromino.render(gl_ctx);
            }
        }

        gl_ctx.transform = old_transform;
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
                render_tetris_block(&position, &tetris_color, gl_ctx);
            }
        }
    }
}

impl Render<Piston2dGraphicsArguments<'_, '_>> for Tetromino {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>) {
        for block in self.blocks() {
            render_tetris_block(block, &self.color(), gl_ctx);
        }
    }
}

/// In [player::render](super::render), helper to render a tetris block at `position` with `tetris_color`.
fn render_tetris_block(
    position: &Position,
    tetris_color: &TetrisColor,
    gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>,
) {
    let dims = rectangle::square(
        position.x() as Scalar * BLOCK_SIZE,
        position.y() as Scalar * BLOCK_SIZE,
        BLOCK_SIZE,
    );

    Image::new().rect(dims).draw(
        gl_ctx.assets.texture_for(tetris_color),
        &gl_ctx.draw_state,
        gl_ctx.transform,
        gl_ctx.gl,
    );
}

/// In [player::render](super::render), helpers used to implement [crate::app::render_app::Render] for [TetrisGrid].
impl TetrisGrid {
    fn total_width(&self) -> f64 {
        self.nb_columns_i32() as f64 * BLOCK_SIZE
    }

    fn visible_height(&self) -> f64 {
        self.nb_matrix_rows_i32() as f64 * BLOCK_SIZE
    }

    fn hidden_height(&self) -> f64 {
        NB_VISIBLE_BUFFER_ROWS as f64 * BLOCK_SIZE
    }
}
