use crate::assets::Assets;
use crate::settings::{NB_NEXT_TETROMINO, BLOCK_SIZE};
use crate::{
    tetris_grid::TetrisGrid, tetromino::Tetromino,
};
use circular_buffer::CircularBuffer;
use graphics::{Context, color, Transformed};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs};

pub struct RemotePlayer {
    grid: TetrisGrid,
    score: u64,
    active_tetromino: Tetromino,
    saved_tetromino: Option<Tetromino>,
    fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>, // push_back / pop_front
}

impl RemotePlayer {
    pub fn render(&mut self, ctx: Context, gl: &mut GlGraphics, args: &RenderArgs, assets: &mut Assets) {
        let score_transform = ctx.transform.trans(0.0, 250.0);
        graphics::text::Text::new_color(color::WHITE, 16)
            .draw(
                format!("Score: {}", self.score).as_str(),
                &mut assets.main_font,
                &ctx.draw_state,
                score_transform,
                gl,
            )
            .unwrap();

        self.grid.render(args, &ctx, gl, assets);

        self.active_tetromino
            .render(self.grid.transform, &ctx, gl, assets);

        if let Some(saved) = self.saved_tetromino {
            let transform = ctx.transform.trans(-70.0, 50.0);
            saved.render(transform, &ctx, gl, assets);
        }

        for i in 0..NB_NEXT_TETROMINO {
            let transform = ctx.transform.trans(BLOCK_SIZE * 16.0, 5.0 * BLOCK_SIZE + 4.0 * BLOCK_SIZE * i as f64);
            self.fifo_next_tetromino.get(i).unwrap().render(transform, &ctx, gl, assets);
        }
    }
}