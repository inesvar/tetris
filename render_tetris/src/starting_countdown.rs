use super::Piston2dOpenGlRenderer;
use core_tetris::{render::RenderTetrisPlayer, Position, TetrisColor, TetrisGrid};

impl Piston2dOpenGlRenderer<'_> {
    pub(super) fn display_starting_countdown(&mut self, grid: &TetrisGrid, color: TetrisColor) {
        let blocks: &[Position] = match self.elapsed_secs {
            0.0..1.0 => &one(grid),
            1.0..2.0 => &two(grid),
            _ => &three(grid),
        };

        for position in blocks {
            self.render_tetris_block(position, color);
        }
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
            grid.nb_matrix_rows_i32() / 2 + y + 2,
        ),
        _ => {
            panic!("x (resp. y) should be between -2 and 2 excluded (resp. -2 and 3 excluded)")
        }
    }
}
