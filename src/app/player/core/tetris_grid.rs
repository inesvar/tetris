//! Define `struct` [TetrisGrid] and `enum` [CoreError].
use super::{spatial_primitives::Position, Deserialize, Serialize, TetrisColor};
use rand::Rng;
use std::ops::{Index, IndexMut};

/// Tetris grid.
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    // Number of columns. Can be cast to `usize` and `u32`.
    nb_columns: i32,
    // Total number of rows (including those above the skyline). Can be cast to `usize` and `u32`.
    nb_rows: i32,
    // Number of hidden rows (above the skyline). Can be cast to `usize` and `u32`.
    nb_hidden_rows: i32,
    matrix: Vec<Vec<Option<TetrisColor>>>,
    line_sum: Vec<i32>,
}

// same visibility as TetrisColor
#[derive(Debug, PartialEq)]
pub(in crate::app) enum CoreError {
    /// Tried to move outside of the grid.
    OutsideOfGrid,
    /// Tried to move to unavailable block.
    UnavailableBlock,
    /// Tried to spawn to unavailable block.
    BlockOut,
    /// Locked down fully above the skyline (cf Guidelines 10.6.).
    LockOut,
}

/// In [crate::app::player::core::tetris_grid], methods used by [crate::app::player] to initialize the grid and send garbage.
impl TetrisGrid {
    /// Create an empty grid.
    pub(in crate::app::player) fn new(nb_columns: u32, nb_rows: u32, nb_hidden_rows: u32) -> Self {
        let nb_rows_usize: usize = nb_rows
            .try_into()
            .expect("nb_rows should be representable as usize");
        let nb_columns_usize: usize = nb_columns
            .try_into()
            .expect("nb_columns should be representable as usize");

        let mut matrix = Vec::with_capacity(nb_rows_usize);
        for _ in 0..nb_rows {
            matrix.push(vec![None; nb_columns_usize]);
        }

        let nb_columns: i32 = nb_columns
            .try_into()
            .expect("nb_columns should be representable as i32");
        let nb_rows: i32 = nb_rows
            .try_into()
            .expect("nb_rows should be representable as i32");
        let nb_hidden_rows: i32 = nb_hidden_rows
            .try_into()
            .expect("nb_hidden_rows should be representable as i32");

        Self {
            nb_columns,
            nb_rows,
            nb_hidden_rows,
            matrix,
            line_sum: vec![0; nb_rows_usize],
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

    /// Adds the specified number of lines at the bottom of the grid. The lines will be filled with blocks except for one column.
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

/// In [crate::app::player::core::tetris_grid], getters used by [crate::app::player::render] to implement [crate::app::render_app::Render] for [TetrisGrid].
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

/// In [crate::app::player::core::tetris_grid], methods used by [super::Tetromino] to enter, move and then lock down in the grid.
impl TetrisGrid {
    /// Return true if the `block` is inside the grid in an empty slot.
    pub(super) fn is_block_available(&self, block: &Position) -> Result<(), CoreError> {
        self.contains(block)?;
        self.is_block_empty(block)
    }

    /// Return true if `blocks` can enter the grid.
    ///
    /// # Panics
    ///
    /// If any of the `blocks` is outside the tetris grid.
    pub(super) fn can_blocks_spawn_on(&self, blocks: &[Position]) -> Result<(), CoreError> {
        let can_spawn = blocks.iter().try_for_each(|b| self.is_block_empty(b));
        can_spawn.map_err(|_| CoreError::BlockOut)
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
    ) -> Result<u64, CoreError> {
        let mut no_block_below_skyline = true;
        for block in blocks {
            self.add_block(block, tetris_color);
            if block.y >= self.nb_hidden_rows {
                no_block_below_skyline = false;
            }
        }
        // Only continue playing if there's a block below the skyline
        if no_block_below_skyline {
            Err(CoreError::LockOut)
        } else {
            Ok(self.clear_lines())
        }
    }
}

impl TetrisGrid {
    /// Return whether `pos` is a valid block inside the tetris grid.
    fn contains(&self, pos: &Position) -> Result<(), CoreError> {
        let is_block_inside_grid =
            pos.x >= 0 && pos.y >= 0 && pos.x < self.nb_columns && pos.y < self.nb_rows;

        if is_block_inside_grid {
            Ok(())
        } else {
            Err(CoreError::OutsideOfGrid)
        }
    }

    /// Return whether `block` is available in the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid.
    fn is_block_empty(&self, block: &Position) -> Result<(), CoreError> {
        if self[block].is_none() {
            Ok(())
        } else {
            Err(CoreError::UnavailableBlock)
        }
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

    /// Clear lines, return number of cleared lines.
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

    /// Add `block` to the tetris grid.
    ///
    /// # Panics
    ///
    /// If `block` is outside the tetris grid or not empty.
    fn add_block(&mut self, block: &Position, tetris_color: TetrisColor) {
        if self.is_block_empty(block).is_err() {
            panic!(
                "Trying to add a block to the grid but {:?} is not empty",
                block
            );
        }
        self[block] = Some(tetris_color);
        self.line_sum[block.y as usize] += 1;
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

    fn tetris_grid_from(str: &[&str]) -> TetrisGrid {
        let nb_rows = str.len();
        let nb_columns = str[0].len();
        let mut tetris_grid = TetrisGrid::new(nb_columns as u32, nb_rows as u32, 0);

        for row in 0..nb_rows {
            for (column, char) in str[row].char_indices() {
                let pos = Position::new(column as i32, row as i32);
                let tetris_color = match char {
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

        for row in 0..nb_rows {
            for (column, char) in str[row].char_indices() {
                let pos = Position::new(column as i32, row as i32);
                if (char == ' ') != tetris_grid.is_block_empty(&pos).is_ok() {
                    return false;
                }
            }
        }

        true
    }

    #[test]
    fn is_block_empty_is_correct() {
        let mut instance = TetrisGrid::new(5, 5, 0);
        let pos = Position::new(0, 0);

        assert!(instance.is_block_empty(&pos).is_ok());

        instance.add_block(&pos, TetrisColor::Grey);

        assert_eq!(
            instance.is_block_empty(&pos),
            Err(CoreError::UnavailableBlock)
        );
    }

    #[test]
    #[should_panic]
    fn is_block_empty_panics_outside_of_the_grid() {
        let instance = TetrisGrid::new(5, 5, 0);
        let pos = Position::new(5, 5);

        let _ = instance.is_block_empty(&pos).is_ok();
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
