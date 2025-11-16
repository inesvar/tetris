//! Define the [render()](PlayerScreen::render()) and [constructor](PlayerScreen::empty()) of [PlayerScreen].
use super::core::{TetrisGrid, Tetromino};
use super::{CircularBuffer, PlayerScreen};
use crate::app::render_app::Render;
use crate::app::ui::text::Text;
use crate::app::Piston2dGraphicsArguments;
use crate::settings::{
    BLOCK_SIZE, DEFAULT_FONT_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, DEFAULT_SCORE_TEXT_Y,
    GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_COLUMNS, NB_HIDDEN_ROWS, NB_NEXT_TETROMINO,
    NB_ROWS, TETROMINO_MAX_HEIGHT, TETROMINO_MAX_WIDTH, TEXT_COLOR,
};
use graphics::{
    types::Rectangle,
    {rectangle, Transformed},
};

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::new(NB_COLUMNS, NB_ROWS, NB_HIDDEN_ROWS), //FIXME: this will not always be the case
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

    // TODO clean this
    pub(in crate::app) fn render(
        &mut self,
        gl_ctx: &mut Piston2dGraphicsArguments,
        display_active_tetromino: bool,
    ) {
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

        if display_active_tetromino {
            self.active_tetromino.render(gl_ctx);
        }

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
                -self.grid.total_width() * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
                TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
            );
            saved.render(gl_ctx);
        }

        // drawing a border for the fifo of next pieces
        gl_ctx.transform = grid_transform.trans(
            self.grid.total_width() * (NB_COLUMNS + 1) as f64 / NB_COLUMNS as f64,
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
                self.grid.total_width() * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
                (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
            );
            if let Some(tetromino) = self.fifo_next_tetromino.get(i) {
                tetromino.render(gl_ctx);
            }
        }

        gl_ctx.transform = old_transform;
    }
}
