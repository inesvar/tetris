//! Define `struct` [TetrisGrid] and `enum` [GameOverError].
use super::{mermaid, spatial_primitives::Position, Deserialize, Serialize, TetrisColor};
use crate::settings::{MAX_SMALL_UNSIGNED, NB_VISIBLE_BUFFER_ROWS};
use rand::Rng;
use std::ops::Index;

/// Tetris grid.
///
/// According to the Tetris Guideline, the grid has 2 components:
/// - **Matrix**: "the rectangular arrangement of cells creating the active game area, usually 10 columns wide by 20 rows high.
///   Tetriminos fall from the top-middle just above the Skyline (off-screen) to the bottom."
/// - **Buffer Zone**: "a 10-cell wide x 20-cell high invisible area above the Matrix used to detect Lock
///   Out, Block Out, and Top Out **Game Over Conditions**."
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    /// Number of columns in the **Matrix** and **Buffer Zone**, has to be between 4 and [MAX_SMALL_UNSIGNED].
    /// Should be 10 according to the Tetris Guideline.
    nb_columns: i32,
    /// Number of rows in the **Matrix**, has to be between 6 and [MAX_SMALL_UNSIGNED].
    /// Should be 20 according to the Tetris Guideline.
    nb_matrix_rows: i32,
    /// Number of buffer rows (ie in the **Buffer Zone** above the **Skyline**), can't be greater than [MAX_SMALL_UNSIGNED].
    /// Should be 20 according to the Tetris Guideline.
    nb_buffer_rows: i32,
    /// **Matrix** and **Buffer Zone** cells, indexed *from bottom to top*.
    cells: Vec<Vec<Option<TetrisColor>>>,
    /// Number of filled blocks in each line of the [TetrisGrid::cells].
    line_sum: Vec<i32>,
}

impl TetrisGrid {
    fn convert_position_y_to_grid_y(&self, y: i32) -> i32 {
        -y + self.nb_matrix_rows + NB_VISIBLE_BUFFER_ROWS as i32 - 1
    }

    fn convert_grid_y_to_position_y(&self, y: i32) -> i32 {
        -y + self.nb_matrix_rows + NB_VISIBLE_BUFFER_ROWS as i32 - 1
    }
}

// same visibility as TetrisColor
/// Tetris Guideline **Game Over Conditions**.
#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub(in crate::app) enum GameOverError {
    /// According to the Tetris Guideline :
    ///
    /// "[...] occurs when part of a newly-generated tetrimino is blocked due to
    /// an existing Block in the Matrix."
    BlockOut,
    /// According to the Tetris Guideline :
    ///
    /// "[...] occurs when a whole tetrimino locks down above the Skyline."
    LockOut,
    #[allow(unused)]
    /// According to the Tetris Guideline :
    ///
    /// "[...] occurs when an opponent’s Line Attack forces your Blocks past the top
    /// of the 20-line Buffer zone. It is highly unlikely that this will ever occur,
    /// since Lock out [...] or Block out [...] will likely occur before a Block ever
    /// gets pushed out of the Buffer zone."
    TopOut,
}

/// In [tetris_grid](super::tetris_grid), methods used by [app::player](crate::app::player)
/// to initialize the grid and send garbage.
impl TetrisGrid {
    /// Create an empty grid.
    ///
    /// # Panics
    ///
    /// If `nb_columns` or `nb_rows` or `nb_hidden_rows` is greater than [MAX_SMALL_UNSIGNED].
    pub(in crate::app::player) fn new(
        nb_columns: u32,
        nb_matrix_rows: u32,
        nb_buffer_rows: u32,
    ) -> Self {
        if nb_columns > MAX_SMALL_UNSIGNED
            || nb_matrix_rows > MAX_SMALL_UNSIGNED
            || nb_buffer_rows > MAX_SMALL_UNSIGNED
        {
            panic!("`nb_columns`, `nb_matrix_rows` and `nb_buffer_rows` should be less than `MAX_SMALL_UNSIGNED`");
        }
        if nb_columns < 4 {
            panic!("`nb_columns` should be greater than or equal to 4");
        }
        if nb_matrix_rows < 6 {
            panic!("`nb_matrix_rows` should be greater than or equal to 6");
        }

        let nb_rows = (nb_matrix_rows + nb_buffer_rows) as usize;
        let mut matrix = Vec::with_capacity(nb_rows);
        for _ in 0..nb_rows {
            matrix.push(vec![None; nb_columns as usize]);
        }

        let nb_columns: i32 = nb_columns as i32;
        let nb_matrix_rows: i32 = nb_matrix_rows as i32;
        let nb_buffer_rows: i32 = nb_buffer_rows as i32;

        Self {
            nb_columns,
            nb_matrix_rows,
            nb_buffer_rows,
            cells: matrix,
            line_sum: vec![0; nb_rows],
        }
    }

    /// Empty the grid.
    pub(in crate::app::player) fn reset(&mut self) {
        *self = Self::new(
            self.nb_columns as u32,
            self.nb_matrix_rows as u32,
            self.nb_buffer_rows as u32,
        );
    }

    /// Add the specified number of lines at the bottom of the grid. The lines will be filled with blocks except for one column.
    pub(in crate::app::player) fn add_garbage(&mut self, completed_lines: u64) {
        if completed_lines < 2 {
            return;
        }
        println!(
            "the garbage creating function was called for {} lines",
            completed_lines
        );
        let lines_to_add = if completed_lines == 4 {
            4
        } else {
            completed_lines - 1
        };

        /*****************************
         *     CHANGING THE GRID     *
         *****************************/

        // store the column index that will be empty
        let mut rng = rand::thread_rng();
        let empty = rng.gen_range(0..self.nb_columns);

        for _ in 0..lines_to_add {
            // move the matrix and line_sum one line up
            self.line_sum.insert(0, self.nb_columns - 1);
            self.line_sum.remove(self.nb_rows());

            self.cells
                .insert(0, vec![Some(TetrisColor::Grey); self.nb_columns as usize]);
            self.cells[0][empty as usize] = None;

            self.cells.remove(self.nb_rows());
        }
    }
}

/// In [tetris_grid](super::tetris_grid), methods used by [UseTetromino](super::UseTetromino)
/// to enter, move and then lock down in the grid.
#[doc = mermaid!("tetris_grid_internals_flowgraph.mmd")]
impl TetrisGrid {
    /// Return true if `blocks` can enter the grid.
    ///
    /// # Panics
    ///
    /// If any of the `blocks` is outside the tetris grid.
    pub(super) fn can_blocks_spawn_on(
        &self,
        blocks: &[Position],
    ) -> Result<Position, GameOverError> {
        let offset = Position::new(self.nb_columns / 2 - 2, 0);
        let can_spawn_on = blocks
            .iter()
            .all(|block| self.is_block_empty(&(*block + offset)));
        if can_spawn_on {
            Ok(offset)
        } else {
            Err(GameOverError::BlockOut)
        }
    }

    /// Return true if the `block` is inside the grid in an empty slot.
    pub(super) fn is_block_available(&self, block: &Position) -> bool {
        self.contains(block) && self.is_block_empty(block)
    }

    /// Push the blocks into the grid and return the number of lines completed.
    ///
    /// # Panics
    ///
    /// If any of the `blocks` is outside the tetris grid or not empty.
    pub(super) fn add_blocks(
        &mut self,
        blocks: &[Position],
        tetris_color: TetrisColor,
    ) -> Result<u64, GameOverError> {
        let all_above_skyline = blocks.iter().all(|block| self.is_above_skyline(block));

        // Only continue playing if there's a block below the skyline
        if all_above_skyline {
            return Err(GameOverError::LockOut);
        }

        for block in blocks {
            self.add_block(block, tetris_color);
        }

        Ok(self.clear_lines())
    }

    /// Return whether `block` is available in the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid.
    fn is_block_empty(&self, block: &Position) -> bool {
        self[block].is_none()
    }

    /// Return `true` is `block` is in the **Matrix** or the **Buffer Zone**.
    fn contains(&self, block: &Position) -> bool {
        let line = self.convert_position_y_to_grid_y(block.y);
        block.x >= 0
            && line >= 0
            && block.x < self.nb_columns
            && line < self.nb_matrix_rows + self.nb_buffer_rows
    }

    /// Remove complete lines, return number of cleared lines.
    fn clear_lines(&mut self) -> u64 {
        let mut score = 0;
        for y in (0..self.nb_rows()).rev() {
            if self.line_sum[y] == self.nb_columns {
                self.pop_row(y);
                score += 1;
            }
        }
        score
    }

    /// Remove `row` from the tetris grid (and add a new empty row at the top).
    ///
    /// # Panics
    ///
    /// If `row` is greater or equal to `self.nb_rows`.
    fn pop_row(&mut self, row: usize) {
        self.cells.remove(row);
        self.line_sum.remove(row);

        self.cells
            .insert(self.nb_rows() - 1, vec![None; self.nb_columns as usize]);
        self.line_sum.insert(self.nb_rows() - 1, 0);
    }

    /// Add `block` to the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid or not empty.
    fn add_block(&mut self, block: &Position, tetris_color: TetrisColor) {
        if !self.is_block_empty(block) {
            panic!()
        }
        let line = self.convert_position_y_to_grid_y(block.y) as usize;
        self.cells[line][block.x as usize] = Some(tetris_color);
        self.line_sum[line] += 1;
    }

    fn is_above_skyline(&self, block: &Position) -> bool {
        self.convert_position_y_to_grid_y(block.y) >= self.nb_matrix_rows
    }

    const fn nb_rows(&self) -> usize {
        (self.nb_matrix_rows + self.nb_buffer_rows) as usize
    }
}

/// In [tetris_grid](super::tetris_grid), helpers used by [player::render](crate::app::player::render)
/// to implement [crate::app::render_app::Render] for [TetrisGrid].
impl TetrisGrid {
    pub(in crate::app::player) const fn nb_matrix_rows(&self) -> i32 {
        self.nb_matrix_rows
    }

    pub(in crate::app::player) const fn nb_columns(&self) -> i32 {
        self.nb_columns
    }

    pub(in crate::app::player) fn positions(&self) -> impl Iterator<Item = Position> + use<'_> {
        let h = self.nb_matrix_rows + self.nb_buffer_rows;
        let w = self.nb_columns;
        (0..h).flat_map(move |y| {
            (0..w).map(move |x| Position::new(x, self.convert_grid_y_to_position_y(y)))
        })
    }

    fn draw_on_empty_grid(&mut self, blocks: &[Position], tetris_color: TetrisColor) {
        self.reset();

        for block in blocks {
            self.add_block(block, tetris_color);
        }
    }

    const fn init(&self, x: i32, y: i32) -> Position {
        match (x, y) {
            (x @ -2..2, y @ -2..3) => Position::new(
                self.nb_columns / 2 + x,
                self.nb_matrix_rows / 2 + y + NB_VISIBLE_BUFFER_ROWS as i32,
            ),
            _ => {
                panic!("x (resp. y) should be between -2 and 2 excluded (resp. -2 and 3 excluded)")
            }
        }
    }

    /// Draw a 1 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn one(&mut self, tetris_color: TetrisColor) {
        let one = [
            self.init(0, -2),
            self.init(-1, -1),
            self.init(0, -1),
            self.init(0, 0),
            self.init(0, 1),
            self.init(-2, 2),
            self.init(-1, 2),
            self.init(0, 2),
            self.init(1, 2),
        ];

        self.draw_on_empty_grid(&one, tetris_color);
    }

    /// Draw a 2 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn two(&mut self, tetris_color: TetrisColor) {
        let two = [
            self.init(-1, -2),
            self.init(0, -2),
            self.init(-2, -1),
            self.init(1, -1),
            self.init(0, 0),
            self.init(-1, 1),
            self.init(-2, 2),
            self.init(-1, 2),
            self.init(0, 2),
            self.init(1, 2),
        ];

        self.draw_on_empty_grid(&two, tetris_color);
    }

    /// Draw a 3 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn three(&mut self, tetris_color: TetrisColor) {
        let three = [
            self.init(-1, -2),
            self.init(0, -2),
            self.init(-2, -1),
            self.init(1, -1),
            self.init(0, 0),
            self.init(-2, 1),
            self.init(1, 1),
            self.init(-1, 2),
            self.init(0, 2),
        ];

        self.draw_on_empty_grid(&three, tetris_color);
    }
}

impl Index<&Position> for TetrisGrid {
    type Output = Option<TetrisColor>;

    fn index(&self, block: &Position) -> &<Self as Index<&Position>>::Output {
        let line = self.convert_position_y_to_grid_y(block.y) as usize;
        &self.cells[line][block.x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{NB_BUFFER_ROWS, NB_COLUMNS, NB_MATRIX_ROWS};

    fn tetris_grid_from(str: &[&str]) -> TetrisGrid {
        let nb_rows = str.len();
        let nb_columns = str[0].len();
        let mut tetris_grid = TetrisGrid::new(nb_columns as u32, nb_rows as u32, 0);

        for (row, line) in str.iter().enumerate() {
            for (column, cell) in line.char_indices() {
                let pos = Position::new(column as i32, row as i32);
                let tetris_color = match cell {
                    ' ' => None,
                    _ => Some(TetrisColor::Grey),
                };
                if let Some(tetris_color) = tetris_color {
                    tetris_grid.add_block(&pos, tetris_color);
                }
            }
        }

        tetris_grid
    }

    fn tetris_grid_matches(tetris_grid: &TetrisGrid, str: &[&str]) -> bool {
        let nb_rows = str.len();
        let nb_columns = str[0].len();

        if tetris_grid.nb_rows() != nb_rows || tetris_grid.nb_columns != nb_columns as i32 {
            return false;
        }

        for (row, line) in str.iter().enumerate() {
            for (column, cell) in line.char_indices() {
                let pos = Position::new(column as i32, row as i32);
                if (cell == ' ') != tetris_grid.is_block_empty(&pos) {
                    return false;
                }
            }
        }

        true
    }

    #[should_panic(
        expected = "`nb_columns`, `nb_rows` and `nb_hidden_rows` should be less than `MAX_SMALL_UNSIGNED`"
    )]
    #[test]
    fn new_fails_if_nb_columns_is_too_big() {
        let _ = TetrisGrid::new(MAX_SMALL_UNSIGNED + 1, 0, 0);
    }

    #[should_panic(
        expected = "`nb_columns`, `nb_rows` and `nb_hidden_rows` should be less than `MAX_SMALL_UNSIGNED`"
    )]
    #[test]
    fn new_fails_if_nb_rows_is_too_big() {
        let _ = TetrisGrid::new(0, MAX_SMALL_UNSIGNED + 1, 0);
    }

    #[should_panic(
        expected = "`nb_columns`, `nb_rows` and `nb_hidden_rows` should be less than `MAX_SMALL_UNSIGNED`"
    )]
    #[test]
    fn new_fails_if_nb_hidden_rows_is_too_big() {
        let _ = TetrisGrid::new(0, 0, MAX_SMALL_UNSIGNED + 1);
    }

    #[test]
    fn new_succeeds() {
        let grid = TetrisGrid::new(NB_COLUMNS, NB_MATRIX_ROWS, NB_BUFFER_ROWS);

        assert_eq!(grid.nb_columns, NB_COLUMNS as i32);
        assert_eq!(grid.nb_matrix_rows, NB_MATRIX_ROWS as i32);
        assert_eq!(grid.nb_buffer_rows, NB_BUFFER_ROWS as i32);
    }

    #[test]
    fn is_block_empty_is_correct() {
        let mut instance = TetrisGrid::new(5, 5, 0);
        let pos = Position::new(0, 0);

        assert!(instance.is_block_empty(&pos));

        instance.add_block(&pos, TetrisColor::Grey);

        assert!(!instance.is_block_empty(&pos));
    }

    #[test]
    #[should_panic]
    fn is_block_empty_panics_outside_of_the_grid() {
        let instance = TetrisGrid::new(5, 5, 0);
        let pos = Position::new(5, 5);

        let _ = instance.is_block_empty(&pos);
    }

    #[test]
    fn pop_row_is_correct() {
        let mut tetris_grid = tetris_grid_from(&[
            "XX XX X X X X",
            " X X X XX XX ",
            "X X XX XX X X",
            "XXXXXXXXXXXXX",
        ]);

        tetris_grid.pop_row(3);

        assert!(tetris_grid_matches(
            &tetris_grid,
            &[
                "             ",
                "XX XX X X X X",
                " X X X XX XX ",
                "X X XX XX X X",
            ]
        ));
    }
}
