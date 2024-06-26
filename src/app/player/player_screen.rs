//! Defines the [render()](PlayerScreen::render()) and [constructor](PlayerScreen::empty()) of [PlayerScreen].
use super::back_end::{TetrisGrid, Tetromino};
use super::{CircularBuffer, PlayerScreen};
use crate::assets::Assets;
use crate::settings::{
    BLOCK_SIZE, DEFAULT_FONT_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, DEFAULT_SCORE_TEXT_Y,
    GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_COLUMNS, NB_NEXT_TETROMINO, NB_ROWS,
    TETROMINO_MAX_HEIGHT, TETROMINO_MAX_WIDTH, TEXT_COLOR,
};
use crate::ui::text::Text;
use graphics::{
    types::{Matrix2d, Rectangle},
    {rectangle, Context, Transformed},
};
use opengl_graphics::GlGraphics;

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::new(DEFAULT_GRID_X, DEFAULT_GRID_Y, NB_COLUMNS, NB_ROWS), //FIXME: this will not always be the case
            score: 0,
            game_over: false,
            new_completed_lines: 0,
            active_tetromino: Tetromino::default(),
            saved_tetromino: None,
            fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new(),
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        }
    }

    pub fn render(
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        let score_text = Text::new(
            format!("Score: {}", self.score).as_str(),
            DEFAULT_FONT_SIZE,
            // the score is centered under the hold piece rectangle
            self.grid.x - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
            DEFAULT_SCORE_TEXT_Y,
            TEXT_COLOR,
        );
        score_text.render(transform, ctx, gl, &mut assets.main_font);

        self.grid.render(transform, &ctx.draw_state, gl, assets);

        if let Some(ghost) = self.ghost_tetromino {
            ghost.render(self.grid.transform, &ctx.draw_state, gl, assets);
        }

        self.active_tetromino
            .render(self.grid.transform, &ctx.draw_state, gl, assets);

        // drawing a border for the hold piece
        let transform = self.grid.transform.trans(
            -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
            self.grid.total_height - self.grid.visible_height,
        );
        let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
        let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
        rectangle(GRID_BG_COLOR, dims, transform, gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &ctx.draw_state, transform, gl);

        // drawing the hold piece
        if let Some(saved) = self.saved_tetromino {
            let transform = self.grid.transform.trans(
                -self.grid.total_width * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            saved.render(transform, &ctx.draw_state, gl, assets);
        }

        // drawing a border for the fifo of next pieces
        let transform = self.grid.transform.trans(
            self.grid.total_width * (NB_COLUMNS + 1) as f64 / NB_COLUMNS as f64,
            self.grid.total_height - self.grid.visible_height,
        );
        let width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
        let height = BLOCK_SIZE + (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * NB_NEXT_TETROMINO as f64;
        let dims: Rectangle = [0.0, 0.0, width, height];
        rectangle(GRID_BG_COLOR, dims, transform, gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
        outline_rect.draw(dims, &ctx.draw_state, transform, gl);

        // drawing the next pieces
        for i in 0..NB_NEXT_TETROMINO {
            let transform = self.grid.transform.trans(
                self.grid.total_width * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = self.fifo_next_tetromino.get(i) {
                tetromino.render(transform, &ctx.draw_state, gl, assets);
            }
        }
    }
}
