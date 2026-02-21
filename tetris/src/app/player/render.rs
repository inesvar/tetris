//! Define `trait` [RenderTetrisCore] for [TetrisPlayer], [Tetromino], [TetrisGrid].
use crate::app::render_app::{Piston2dOpenGlRenderer, RenderTetrisCore, RenderTetrisUi};
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

impl RenderTetrisCore for Piston2dOpenGlRenderer<'_> {
    fn render_player(&mut self, player: &TetrisPlayer) {
        let score_text = Text::new(
            format!("Score: {}", player.score).as_str(),
            DEFAULT_FONT_SIZE,
            // the score is centered under the hold piece rectangle
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
            DEFAULT_SCORE_TEXT_Y,
            TEXT_COLOR,
        );
        self.render_text(&score_text);

        let old_transform = self.transform;
        let grid_transform = self.transform.trans(DEFAULT_GRID_X, DEFAULT_GRID_Y);
        self.transform = grid_transform;

        if !player.is_in_play() {
            self.render_tetromino(&player.tetromino_in_play);
        }

        self.render_tetris_grid(&player.matrix);

        if player.is_in_play() {
            self.render_tetromino(&player.tetromino_in_play);

            let old_draw_state = self.draw_state;
            self.draw_state = self.draw_state.blend(graphics::draw_state::Blend::Multiply);
            self.render_tetromino(&player.get_ghost_tetromino());
            self.draw_state = old_draw_state;
        }

        // drawing a border for the hold piece
        self.transform = grid_transform.trans(
            -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
            hidden_height(&player.matrix),
        );
        let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
        let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
        rectangle(GRID_BG_COLOR, dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &self.draw_state, self.transform, &mut self.gl);

        // drawing the hold piece
        if let Some(saved) = &player.hold_queue {
            self.transform = grid_transform.trans(
                -TETROMINO_MAX_WIDTH - 2.0 * BLOCK_SIZE,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            self.render_tetromino(saved);
        }

        // drawing a border for the fifo of next pieces
        self.transform = grid_transform.trans(
            total_width(&player.matrix) + BLOCK_SIZE,
            hidden_height(&player.matrix),
        );
        let width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let height = BLOCK_SIZE + (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * NB_NEXT_TETROMINO as f64;
        let dims: Rectangle = [0.0, 0.0, width, height];
        rectangle(GRID_BG_COLOR, dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &self.draw_state, self.transform, &mut self.gl);

        // drawing the next pieces
        for i in 0..NB_NEXT_TETROMINO {
            self.transform = grid_transform.trans(
                total_width(&player.matrix) + 2.0 * BLOCK_SIZE,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = player.next_queue.get(i) {
                self.render_tetromino(tetromino);
            }
        }

        self.transform = old_transform;
    }

    fn render_tetris_grid(&mut self, grid: &TetrisGrid) {
        let empty_dims: Rectangle = [
            0.0,
            hidden_height(grid),
            total_width(grid),
            visible_height(grid),
        ];
        rectangle(GRID_BG_COLOR, empty_dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(empty_dims, &self.draw_state, self.transform, &mut self.gl);

        for position in grid.positions() {
            if let Some(tetris_color) = grid[&position] {
                self.render_tetris_block(&position, tetris_color);
            }
        }
    }

    fn render_tetromino(&mut self, tetromino: &Tetromino) {
        for block in tetromino.blocks() {
            self.render_tetris_block(block, tetromino.color());
        }
    }

    /// In [player::render](super::render), helper to render a tetris block at `position` with `tetris_color`.
    fn render_tetris_block(&mut self, position: &Position, tetris_color: TetrisColor) {
        let dims = rectangle::square(
            position.x() as Scalar * BLOCK_SIZE,
            position.y() as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            self.assets.texture_for(&tetris_color),
            &self.draw_state,
            self.transform,
            &mut self.gl,
        );
    }
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
