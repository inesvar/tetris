use crate::assets::Assets;
use crate::circular_buffer::CircularBuffer;
use crate::settings::{
    BLOCK_SIZE, GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_COLUMNS, NB_NEXT_TETROMINO, NB_ROWS,
};
use crate::ui::text::Text;
use crate::{tetris_grid::TetrisGrid, tetromino::Tetromino};

use graphics::types::{Matrix2d, Rectangle};
use graphics::{color, rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerScreen {
    pub grid: TetrisGrid,
    pub score: u64,
    pub new_completed_lines: u64,
    pub active_tetromino: Tetromino,
    pub saved_tetromino: Option<Tetromino>,
    pub fifo_next_tetromino: CircularBuffer<NB_NEXT_TETROMINO, Tetromino>,
    pub ghost_tetromino: Option<Tetromino>,
}

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::new(140.0, 70.0, NB_COLUMNS, NB_ROWS), //FIXME: this will not always be the case
            score: 0,
            new_completed_lines: 0,
            active_tetromino: Tetromino::default(),
            saved_tetromino: None,
            fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new(),
            ghost_tetromino: None,
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
            16,
            100.0,
            250.0,
            color::WHITE,
        );
        score_text.render(transform, &ctx, gl, &mut assets.main_font);

        self.grid.render(transform, &ctx, gl, assets);

        if let Some(ghost) = self.ghost_tetromino {
            ghost.render(self.grid.transform, &ctx, gl, assets);
        }

        self.active_tetromino
            .render(self.grid.transform, &ctx, gl, assets);

        if let Some(saved) = self.saved_tetromino {
            let transform = self.grid.transform.trans(
                -4.0 * BLOCK_SIZE - (saved.center.x as f64 * BLOCK_SIZE),
                2.0 * BLOCK_SIZE,
            );
            saved.render(transform, &ctx, gl, assets);
        }

        let transform = self.grid.transform.trans(
            self.grid.total_width * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
            2.0 * BLOCK_SIZE,
        );
        let height = (1.0 + 3.0 * NB_NEXT_TETROMINO as f64) * BLOCK_SIZE;
        let dims: Rectangle = [2.0 * BLOCK_SIZE, 0.0, 6.0 * BLOCK_SIZE, height];
        rectangle(GRID_BG_COLOR, dims, transform, gl);
        let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
        outline_rect.draw(dims, &ctx.draw_state, transform, gl);

        for i in 0..NB_NEXT_TETROMINO {
            let transform = self.grid.transform.trans(
                self.grid.total_width * (NB_COLUMNS - 1) as f64 / NB_COLUMNS as f64,
                3.0 * BLOCK_SIZE * (i as f64 + 1.0),
            );
            self.fifo_next_tetromino
                .get(i)
                .unwrap()
                .render(transform, &ctx, gl, assets);
        }
    }
}
