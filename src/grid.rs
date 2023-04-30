use opengl_graphics::GlGraphics;
use crate::block::Block;
use crate::grid::GridCell::Empty;

pub enum GridCell {
    Empty,
    Block(Block)
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<Vec<GridCell>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Empty);
            }
            rows.push(row);
        }
        Grid {
            width,
            height,
            rows
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        for row in &self.rows {
            for cell in row {
                match cell {
                    Empty => {},
                    Block(block) => block.render(gl)
                }
            }
        }
    }
}