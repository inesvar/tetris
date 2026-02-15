//! Define `trait` [Render] for [LocalPlayer], [TetrisPlayer], [Tetromino], [TetrisGrid].
use crate::app::player::LocalPlayer;
use crate::app::render_app::{Piston2dGraphicsArguments, Render};
use crate::app::ui::text::Text;
use crate::app::TetrisPlayer;
use crate::settings::{
    BLOCK_SIZE, DEFAULT_FONT_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, DEFAULT_SCORE_TEXT_Y,
    GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_NEXT_TETROMINO, TETROMINO_MAX_HEIGHT,
    TETROMINO_MAX_WIDTH, TEXT_COLOR,
};
use graphics::types::{Rectangle, Scalar};
use graphics::{rectangle, Image, Transformed};
use tetris_core::{Position, TetrisColor, TetrisGrid, Tetromino, NB_VISIBLE_BUFFER_ROWS};

impl Render<Piston2dGraphicsArguments<'_, '_>> for LocalPlayer {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        self.player_screen.render(gl_ctx);
    }
}

impl Render<Piston2dGraphicsArguments<'_, '_>> for TetrisPlayer {
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

        if !self.is_in_play() {
            self.tetromino_in_play.render(gl_ctx);
        }

        self.matrix.render(gl_ctx);

        if self.is_in_play() {
            self.tetromino_in_play.render(gl_ctx);

            let old_draw_state = gl_ctx.draw_state;
            gl_ctx.draw_state = gl_ctx
                .draw_state
                .blend(graphics::draw_state::Blend::Multiply);
            self.get_ghost_tetromino().render(gl_ctx);
            gl_ctx.draw_state = old_draw_state;
        }

        // drawing a border for the hold piece
        gl_ctx.transform = grid_transform.trans(
            -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
            hidden_height(&self.matrix),
        );
        let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
        let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
        rectangle(GRID_BG_COLOR, dims, gl_ctx.transform, gl_ctx.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

        // drawing the hold piece
        if let Some(saved) = &self.hold_queue {
            gl_ctx.transform = grid_transform.trans(
                -TETROMINO_MAX_WIDTH - 2.0 * BLOCK_SIZE,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            saved.render(gl_ctx);
        }

        // drawing a border for the fifo of next pieces
        gl_ctx.transform = grid_transform.trans(
            total_width(&self.matrix) + BLOCK_SIZE,
            hidden_height(&self.matrix),
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
                total_width(&self.matrix) + 2.0 * BLOCK_SIZE,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = self.next_queue.get(i) {
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
            hidden_height(self),
            total_width(self),
            visible_height(self),
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

fn total_width(grid: &TetrisGrid) -> f64 {
    grid.nb_columns_i32() as f64 * BLOCK_SIZE
}

fn visible_height(grid: &TetrisGrid) -> f64 {
    grid.nb_matrix_rows_i32() as f64 * BLOCK_SIZE
}

fn hidden_height(_grid: &TetrisGrid) -> f64 {
    NB_VISIBLE_BUFFER_ROWS as f64 * BLOCK_SIZE
}
