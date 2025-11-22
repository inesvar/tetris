//! Define `struct` [TetrisGrid] and `enum` [GameOverError].
use super::{mermaid, spatial_primitives::Position, Deserialize, Serialize, TetrisColor};
use crate::settings::MAX_SMALL_UNSIGNED;
use rand::Rng;
use std::ops::{Index, IndexMut};

/// Tetris grid.
///
/// According to the Tetris Guideline, the grid has 2 components:
/// - **Matrix**: "the rectangular arrangement of cells creating the active game area, usually 10 columns wide by 20 rows high.
///   Tetriminos fall from the top-middle just above the Skyline (off-screen) to the bottom."
/// - **Buffer Zone**: "a 10-cell wide x 20-cell high invisible area above the Matrix used to detect Lock
///   Out, Block Out, and Top Out **Game Over Conditions**."
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    /// Number of columns in the **Matrix** and **Buffer Zone**, can't be greater than [MAX_SMALL_UNSIGNED].
    /// Should be 10 according to the Tetris Guideline.
    nb_columns: i32,
    /// Total number of rows in the **Matrix** and **Buffer Zone**, can't be greater than [MAX_SMALL_UNSIGNED].
    /// Should be 40 according to the Tetris Guideline.
    nb_rows: i32,
    /// Number of hidden rows (ie in the **Buffer Zone** above the Skyline), can't be greater than [MAX_SMALL_UNSIGNED].
    /// Should be 20 according to the Tetris Guideline.
    nb_hidden_rows: i32,
    /// **Matrix** and **Buffer Zone** cells, indexed *from top to bottom*.
    matrix: Vec<Vec<Option<TetrisColor>>>,
    /// Number of filled blocks in each line of the [TetrisGrid::matrix].
    line_sum: Vec<i32>,
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
    pub(in crate::app::player) fn new(nb_columns: u32, nb_rows: u32, nb_hidden_rows: u32) -> Self {
        if nb_columns > MAX_SMALL_UNSIGNED
            || nb_rows > MAX_SMALL_UNSIGNED
            || nb_hidden_rows > MAX_SMALL_UNSIGNED
        {
            panic!("`nb_columns`, `nb_rows` and `nb_hidden_rows` should be less than `MAX_SMALL_UNSIGNED`");
        }

        let mut matrix = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            matrix.push(vec![None; nb_columns as usize]);
        }

        let nb_columns: i32 = nb_columns as i32;
        let nb_rows: i32 = nb_rows as i32;
        let nb_hidden_rows: i32 = nb_hidden_rows as i32;

        Self {
            nb_columns,
            nb_rows,
            nb_hidden_rows,
            matrix,
            line_sum: vec![0; nb_rows as usize],
        }
    }

    /// Empty the grid.
    pub(in crate::app::player) fn reset(&mut self) {
        *self = Self::new(
            self.nb_columns as u32,
            self.nb_rows as u32,
            self.nb_hidden_rows as u32,
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
            self.line_sum
                .insert(self.nb_rows as usize, self.nb_columns - 1);
            self.line_sum.remove(0);

            self.matrix
                .insert(self.nb_rows as usize, vec![None; self.nb_columns as usize]);
            for x in 0..self.nb_columns {
                // add blocks in the entire line except in one column
                if x != empty {
                    self.matrix[self.nb_rows as usize][x as usize] = Some(TetrisColor::Grey);
                } else {
                    self.matrix[self.nb_rows as usize][x as usize] = None;
                }
            }
            self.matrix.remove(0);
        }
    }
}

/// In [tetris_grid](super::tetris_grid), methods used by [UseTetromino](super::UseTetromino)
/// to enter, move and then lock down in the grid.
#[doc = mermaid!("use_tetromino_flowgraph.mmd")]
impl TetrisGrid {
    /// Return true if `blocks` can enter the grid.
    ///
    /// # Panics
    ///
    /// If any of the `blocks` is outside the tetris grid.
    pub(super) fn can_blocks_spawn_on(&self, blocks: &[Position]) -> Result<(), GameOverError> {
        let can_spawn_on = blocks.iter().all(|block| self.is_block_empty(block));
        if can_spawn_on {
            Ok(())
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
    /// If any of the `blocks` is outside the tetris grid.
    pub(super) fn add_blocks(
        &mut self,
        blocks: &[Position],
        tetris_color: TetrisColor,
    ) -> Result<u64, GameOverError> {
        let mut no_block_below_skyline = true;
        for block in blocks {
            self.add_block(block, tetris_color);
            if block.y >= self.nb_hidden_rows {
                no_block_below_skyline = false;
            }
        }
        // Only continue playing if there's a block below the skyline
        if no_block_below_skyline {
            Err(GameOverError::LockOut)
        } else {
            Ok(self.clear_lines())
        }
    }

    /// Return whether `block` is available in the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid.
    fn is_block_empty(&self, block: &Position) -> bool {
        self[block].is_none()
    }

    /// Return whether `pos` is a valid block inside the tetris grid.
    fn contains(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.nb_columns && pos.y < self.nb_rows
    }

    /// Remove complete lines, return number of cleared lines.
    fn clear_lines(&mut self) -> u64 {
        let mut score = 0;
        for y in 0..self.nb_rows as usize {
            if self.line_sum[y] == self.nb_columns {
                self.pop_row(y);
                score += 1;
            }
        }
        score
    }

    /// Remove `row` from the tetris grid.
    ///
    /// # Panics
    ///
    /// If `row` is greater or equal to `self.nb_rows`.
    fn pop_row(&mut self, row: usize) {
        self.matrix.remove(row);
        self.line_sum.remove(row);

        self.matrix.insert(0, vec![None; self.nb_columns as usize]);
        self.line_sum.insert(0, 0);
    }

    /// Add `block` to the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid or not empty.
    fn add_block(&mut self, block: &Position, tetris_color: TetrisColor) {
        if !self.is_block_empty(block) {
            panic!(
                "Trying to add a block to the grid but {:?} is not empty",
                block
            );
        }
        self[block] = Some(tetris_color);
        self.line_sum[block.y as usize] += 1;
    }
}

/// In [tetris_grid](super::tetris_grid), getters used by [player::render](crate::app::player::render)
/// to implement [crate::app::render_app::Render] for [TetrisGrid].
impl TetrisGrid {
    pub(in crate::app::player) fn nb_visible_rows(&self) -> i32 {
        self.nb_rows - self.nb_hidden_rows
    }

    pub(in crate::app::player) fn nb_hidden_rows(&self) -> i32 {
        self.nb_hidden_rows
    }

    pub(in crate::app::player) fn nb_columns(&self) -> i32 {
        self.nb_columns
    }

    pub(in crate::app::player) fn positions(&self) -> impl Iterator<Item = Position> {
        let h = self.nb_rows;
        let w = self.nb_columns;
        (0..h).flat_map(move |y| (0..w).map(move |x| Position::new(x, y)))
    }
}

impl Index<&Position> for TetrisGrid {
    type Output = Option<TetrisColor>;

    fn index(&self, block: &Position) -> &<Self as Index<&Position>>::Output {
        &self.matrix[block.y as usize][block.x as usize]
    }
}

impl IndexMut<&Position> for TetrisGrid {
    fn index_mut(&mut self, block: &Position) -> &mut <Self as Index<&Position>>::Output {
        &mut self.matrix[block.y as usize][block.x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{NB_COLUMNS, NB_HIDDEN_ROWS, NB_ROWS};

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

        if tetris_grid.nb_rows != nb_rows as i32 || tetris_grid.nb_columns != nb_columns as i32 {
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
        let grid = TetrisGrid::new(NB_COLUMNS, NB_ROWS, NB_HIDDEN_ROWS);

        assert_eq!(grid.nb_columns, NB_COLUMNS as i32);
        assert_eq!(grid.nb_rows, NB_ROWS as i32);
        assert_eq!(grid.nb_hidden_rows, NB_HIDDEN_ROWS as i32);
    }

    #[test]
    fn is_block_empty_is_correct() {
        let mut instance = TetrisGrid::new(5, 5, 0);
        let pos = Position::new(0, 0);

        assert!(instance.is_block_empty(&pos));

        instance.add_block(&pos, TetrisColor::Grey);

        assert!(
            !instance.is_block_empty(&pos)
        );
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
