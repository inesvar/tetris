//! Define `struct` [TetrisGrid].
use super::{spatial_primitives::Position, Deserialize, Serialize, TetrisColor};
use rand::Rng;
use std::ops::{Index, IndexMut};

type GridLine = Vec<Option<TetrisColor>>;

/// Tetris grid.
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    pub(super) nb_columns: i32,
    pub(super) nb_rows: i32,
    pub(super) nb_hidden_rows: i32,
    pub(super) matrix: Vec<GridLine>,
    line_sum: Vec<i32>,
}

// same visibility as TetrisColor
#[derive(Debug, PartialEq)]
pub(in crate::app) enum BackendError {
    TriedToMoveOutsideOfGrid,
    TriedToMoveToUnavailableBlock,
    /// cf Guidelines 10.6. : when a whole tetromino locks down above the skyline
    LockOut,
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

impl TetrisGrid {
    pub(in crate::app::player) fn new(
        nb_columns: u32,
        nb_rows: u32,
        nb_hidden_rows: u32,
    ) -> TetrisGrid {
        let nb_columns: i32 = nb_columns
            .try_into()
            .expect("nb_columns should be representable as i32");
        let nb_rows: i32 = nb_rows
            .try_into()
            .expect("nb_rows should be representable as i32");
        let nb_hidden_rows: i32 = nb_hidden_rows
            .try_into()
            .expect("nb_hidden_rows should be representable as i32");

        let mut matrix = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            matrix.push(vec![None; nb_columns as usize]);
        }

        let line_sum = vec![0; nb_rows as usize];
        TetrisGrid {
            nb_columns,
            nb_rows,
            nb_hidden_rows,
            matrix,
            line_sum,
        }
    }

    fn is_block_inside_grid(&self, block: &Position) -> Result<(), BackendError> {
        let is_block_inside_grid =
            block.x >= 0 && block.y >= 0 && block.x < self.nb_columns && block.y < self.nb_rows;

        if is_block_inside_grid {
            Ok(())
        } else {
            Err(BackendError::TriedToMoveOutsideOfGrid)
        }
    }

    pub(super) fn is_block_empty(&self, block: &Position) -> Result<(), BackendError> {
        if self[block].is_none() {
            Ok(())
        } else {
            Err(BackendError::TriedToMoveToUnavailableBlock)
        }
    }

    /// Returns true if the `block` is inside the grid in an empty slot.
    pub(super) fn is_block_available(&self, block: &Position) -> Result<(), BackendError> {
        self.is_block_inside_grid(block)?;

        self.is_block_empty(block)
    }

    /// Returns true if the `blocks` are inside the grid on empty slots.
    pub(super) fn are_blocks_available(&self, blocks: &[Position]) -> bool {
        blocks.iter().all(|b| self.is_block_available(b).is_ok())
    }

    fn pop_row(&mut self, row: usize) {
        self.matrix.remove(row);
        self.matrix.insert(0, vec![None; self.nb_columns as usize]);

        self.line_sum.remove(row);
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

    fn add_block(&mut self, block: &Position, tetris_color: TetrisColor) {
        self[block] = Some(tetris_color);
        self.line_sum[block.y as usize] += 1;
    }

    /// Push the blocks into the grid and return the number of lines completed.
    pub(super) fn add_blocks(
        &mut self,
        blocks: &[Position],
        tetris_color: TetrisColor,
    ) -> Result<u64, BackendError> {
        let mut no_block_below_skyline = true;
        for block in blocks {
            self.add_block(block, tetris_color);
            if block.y >= self.nb_hidden_rows {
                no_block_below_skyline = false;
            }
        }
        // Only continue playing if there's a block below the skyline
        if no_block_below_skyline {
            Err(BackendError::LockOut)
        } else {
            Ok(self.clear_lines())
        }
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

    /// Empty the grid.
    pub(in crate::app::player) fn null(&mut self) {
        for row in self.matrix.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
        self.line_sum = vec![0; self.nb_rows as usize];
    }
}
