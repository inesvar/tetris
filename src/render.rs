use graphics::{Context, DrawState, Image, rectangle};
use graphics::draw_state::Blend;
use graphics::types::{Matrix2d, Rectangle, Scalar};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use crate::{Assets, Tetromino};
use crate::block::Block;
use crate::settings::{BLOCK_SIZE, GRID_COLOR, GRID_THICKNESS};
use crate::tetris_grid::TetrisGrid;
use crate::graphics::Transformed;

// TODO : réfléchir à une façon de la rendre &self au lieu de &mut self
impl TetrisGrid {
    pub fn render(
        &mut self,
        args: &RenderArgs,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        self.transform = ctx.transform.trans(
            args.window_size[0] / 2.0 - self.total_width / 2.0,
            args.window_size[1] / 2.0 - self.total_height / 2.0,
        );

        let empty_dims: Rectangle = [
            0.0,
            self.total_height - self.visible_height,
            self.visible_width,
            self.visible_height,
        ];
        rectangle([0.1, 0.1, 0.1, 1.0], empty_dims, self.transform, gl);

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if y > 1 {
                    let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
                    let outline_dims = rectangle::square(
                        x as Scalar * BLOCK_SIZE,
                        y as Scalar * BLOCK_SIZE,
                        BLOCK_SIZE,
                    );
                    outline_rect.draw(outline_dims, &ctx.draw_state, self.transform, gl);

                    match cell {
                        Some(block) => block.render(self.transform, &ctx.draw_state, gl, assets),
                        None => {}
                    }
                }
            }
        }
    }
}

impl Tetromino {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        for i in 0..4 {
            let draw_state = if self.is_ghost {
                ctx.draw_state.blend(Blend::Multiply)
            } else {
                ctx.draw_state
            };
            self.blocks[i].render(transform, &draw_state, gl, assets);
        }
    }
}

impl Block {
    pub fn render(
        &self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let dims = rectangle::square(
            self.position.x as Scalar * BLOCK_SIZE,
            self.position.y as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            assets.texture_from_tetris_color(&self.color),
            draw_state,
            transform,
            gl,
        );
    }
}