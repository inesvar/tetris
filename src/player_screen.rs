use crate::assets::Assets;
use crate::circular_buffer::CircularBuffer;
use crate::settings::{NB_COLUMNS, NB_NEXT_TETROMINO, NB_ROWS, BLOCK_SIZE};
use crate::ui::text::Text;
use crate::{tetris_grid::TetrisGrid, tetromino::Tetromino};

use graphics::{Context, Transformed, color};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerScreen {
    pub grid: TetrisGrid,
    pub score: u64,
    pub active_tetromino: Tetromino,
    pub saved_tetromino: Option<Tetromino>,
    pub fifo_next_tetromino: CircularBuffer<NB_NEXT_TETROMINO, Tetromino>,
    pub ghost_tetromino: Option<Tetromino>,
}

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::new(150.0, 70.0, NB_COLUMNS, NB_ROWS), //FIXME: this will not always be the case
            score: 0,
            active_tetromino: Tetromino::default(),
            saved_tetromino: None,
            fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new(),
            ghost_tetromino: None,
        }
    }

    pub fn render(&mut self, ctx: Context, gl: &mut GlGraphics, assets: &mut Assets) {
        let score_text = Text::new(format!("Score: {}", self.score), 16, 0.0, 250.0, color::WHITE);
        score_text.render(ctx.transform, &ctx, gl, &mut assets.main_font);

        self.grid.render(ctx.transform, &ctx, gl, assets);

        if let Some(ghost) = self.ghost_tetromino {
            ghost.render(self.grid.transform, &ctx, gl, assets);
        }

        self.active_tetromino
            .render(self.grid.transform, &ctx, gl, assets);

        if let Some(saved) = self.saved_tetromino {
            let transform = self.grid.transform.trans(-100.0 - (saved.center.x as f64 * BLOCK_SIZE), 50.0);
            saved.render(transform, &ctx, gl, assets);
        }

        for i in 0..NB_NEXT_TETROMINO {
            let transform = self.grid.transform.trans(
                self.grid.total_width,
                4.0 * BLOCK_SIZE * (i as f64 + 0.5),
            );
            self.fifo_next_tetromino
                .get(i)
                .unwrap()
                .render(transform, &ctx, gl, assets);
        }
    }
}
