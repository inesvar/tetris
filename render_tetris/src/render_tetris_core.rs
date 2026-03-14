//! Implement `trait` [RenderTetrisCore] for [Piston2dOpenGlRenderer].
use super::Piston2dOpenGlRenderer;
use super::{
    BLOCK_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS,
    TETROMINO_MAX_HEIGHT, TETROMINO_MAX_WIDTH,
};
use core_tetris::{
    Position, RenderTetrisCore, RunningState, TetrisColor, TetrisGrid, TetrisPlayer,
    NB_VISIBLE_BUFFER_ROWS,
};
use graphics::types::{Rectangle, Scalar};
use graphics::{rectangle, Image, Transformed};

impl RenderTetrisCore for Piston2dOpenGlRenderer<'_> {
    fn display_player(&mut self, player: &TetrisPlayer, state: RunningState) {
        /* let score_text = Text::new(
            format!("Score: {}", player.score).as_str(),
            DEFAULT_FONT_SIZE,
            // the score is centered under the hold piece rectangle
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
            DEFAULT_SCORE_TEXT_Y,
            TEXT_COLOR,
        );
        self.render_text(&score_text); */

        let old_transform = self.transform;
        let grid_transform = self.transform.trans(DEFAULT_GRID_X, DEFAULT_GRID_Y);
        self.transform = grid_transform;

        self.display_tetris_grid(player, state);

        self.display_tetromino_in_play(player, state);

        self.display_hold_queue(player, state);

        self.display_next_queue(player, state);

        self.transform = old_transform;
    }

    fn display_tetris_grid(&mut self, player: &TetrisPlayer, state: RunningState) {
        let grid = &player.grid;
        let empty_dims: Rectangle = [
            0.0,
            hidden_height(grid),
            total_width(grid),
            visible_height(grid),
        ];
        rectangle(GRID_BG_COLOR, empty_dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(empty_dims, &self.draw_state, self.transform, &mut self.gl);

        if state != RunningState::Starting {
            self.render_tetris_grid(grid);
        } else {
            let blocks: &[Position] = match self.elapsed_secs {
                0.0..1.0 => &one(grid),
                1.0..2.0 => &two(grid),
                _ => &three(grid),
            };

            for position in blocks {
                self.render_tetris_block(position, player.tetromino_in_play.color());
            }
        }
    }

    fn display_tetromino_in_play(&mut self, player: &TetrisPlayer, state: RunningState) {
        let tetromino = &player.tetromino_in_play;

        if state == RunningState::Running {
            self.render_tetromino(tetromino);

            let old_draw_state = self.draw_state;
            self.draw_state = self.draw_state.blend(graphics::draw_state::Blend::Multiply);
            self.render_tetromino(&player.get_ghost_tetromino());
            self.draw_state = old_draw_state;
        } else if (self.elapsed_secs * 2.0) % 2.0 < 1.0 {
            self.render_tetromino(tetromino);
        }
    }

    fn display_hold_queue(&mut self, player: &TetrisPlayer, _: RunningState) {
        // drawing a border for the hold piece
        let old_transform = self.transform;
        self.transform = self.transform.trans(
            -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
            hidden_height(&player.grid),
        );
        let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
        let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
        rectangle(GRID_BG_COLOR, dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &self.draw_state, self.transform, &mut self.gl);

        // drawing the hold piece
        if let Some(saved) = &player.hold_queue {
            self.transform = old_transform.trans(
                -TETROMINO_MAX_WIDTH - 2.0 * BLOCK_SIZE,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            self.render_tetromino(saved);
        }

        self.transform = old_transform;
    }

    fn display_next_queue(&mut self, player: &TetrisPlayer, _: RunningState) {
        // drawing a border for the fifo of next pieces
        let old_transform = self.transform;

        self.transform = self.transform.trans(
            total_width(&player.grid) + BLOCK_SIZE,
            hidden_height(&player.grid),
        );
        let width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let height =
            BLOCK_SIZE + (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * player.next_queue.size() as f64;
        let dims: Rectangle = [0.0, 0.0, width, height];
        rectangle(GRID_BG_COLOR, dims, self.transform, &mut self.gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &self.draw_state, self.transform, &mut self.gl);

        // drawing the next pieces
        for i in 0..player.next_queue.size() {
            self.transform = old_transform.trans(
                total_width(&player.grid) + 2.0 * BLOCK_SIZE,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = player.next_queue.get(i) {
                self.render_tetromino(tetromino);
            }
        }

        self.transform = old_transform;
    }

    /// Helper to render a tetris block at `position` with `tetris_color`.
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

fn one(grid: &TetrisGrid) -> [Position; 9] {
    [
        center_in_grid(grid, 0, -2),
        center_in_grid(grid, -1, -1),
        center_in_grid(grid, 0, -1),
        center_in_grid(grid, 0, 0),
        center_in_grid(grid, 0, 1),
        center_in_grid(grid, -2, 2),
        center_in_grid(grid, -1, 2),
        center_in_grid(grid, 0, 2),
        center_in_grid(grid, 1, 2),
    ]
}

fn two(grid: &TetrisGrid) -> [Position; 10] {
    [
        center_in_grid(grid, -1, -2),
        center_in_grid(grid, 0, -2),
        center_in_grid(grid, -2, -1),
        center_in_grid(grid, 1, -1),
        center_in_grid(grid, 0, 0),
        center_in_grid(grid, -1, 1),
        center_in_grid(grid, -2, 2),
        center_in_grid(grid, -1, 2),
        center_in_grid(grid, 0, 2),
        center_in_grid(grid, 1, 2),
    ]
}

fn three(grid: &TetrisGrid) -> [Position; 9] {
    [
        center_in_grid(grid, -1, -2),
        center_in_grid(grid, 0, -2),
        center_in_grid(grid, -2, -1),
        center_in_grid(grid, 1, -1),
        center_in_grid(grid, 0, 0),
        center_in_grid(grid, -2, 1),
        center_in_grid(grid, 1, 1),
        center_in_grid(grid, -1, 2),
        center_in_grid(grid, 0, 2),
    ]
}

fn center_in_grid(grid: &TetrisGrid, x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ -2..2, y @ -2..3) => Position::new(
            grid.nb_columns_i32() / 2 + x,
            grid.nb_matrix_rows_i32() / 2 + y + NB_VISIBLE_BUFFER_ROWS as i32,
        ),
        _ => {
            panic!("x (resp. y) should be between -2 and 2 excluded (resp. -2 and 3 excluded)")
        }
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
