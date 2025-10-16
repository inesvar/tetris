//! Define `struct` [TetrisGrid].
use super::{tetris_block::Block, Deserialize, Serialize, TetrisColor, Tetromino};
use crate::settings::BLOCK_SIZE;
use rand::Rng;
use std::ops::{Index, IndexMut};

type GridLine = Vec<Option<TetrisColor>>;

/// Tetris grid.
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    nb_columns: u32,
    nb_rows: u32,
    nb_hidden_rows: u32,
    pub(super) matrix: Vec<GridLine>,
    line_sum: Vec<u32>,
}

// same visibility as TetrisColor
#[derive(Debug, PartialEq)]
pub enum BackendError {
    TriedToMoveOutsideOfGrid,
    TriedToMoveToUnavailableBlock,
}

impl Index<&Block> for TetrisGrid {
    type Output = Option<TetrisColor>;

    fn index(&self, block: &Block) -> &<Self as Index<&Block>>::Output {
        &self.matrix[block.y() as usize][block.x() as usize]
    }
}

impl IndexMut<&Block> for TetrisGrid {
    fn index_mut(&mut self, block: &Block) -> &mut <Self as Index<&Block>>::Output {
        &mut self.matrix[block.y() as usize][block.x() as usize]
    }
}

impl TetrisGrid {
    pub(in crate::app::player) fn new(
        nb_columns: u32,
        nb_rows: u32,
        nb_hidden_rows: u32,
    ) -> TetrisGrid {
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

    fn is_block_inside_grid(&self, block: &Block) -> Result<(), BackendError> {
        let is_block_inside_grid = block.x() >= 0
            && block.y() >= 0
            && (block.x() as u32) < self.nb_columns
            && (block.y() as u32) < self.nb_rows;

        if is_block_inside_grid {
            Ok(())
        } else {
            Err(BackendError::TriedToMoveOutsideOfGrid)
        }
    }

    fn is_block_empty(&self, block: &Block) -> Result<(), BackendError> {
        if self[block].is_none() {
            Ok(())
        } else {
            Err(BackendError::TriedToMoveToUnavailableBlock)
        }
    }

    /// Returns true if the `block` is inside the grid in an empty slot.
    pub(super) fn is_block_available(&self, block: &Block) -> Result<(), BackendError> {
        self.is_block_inside_grid(block)?;

        self.is_block_empty(block)
    }

    /// Returns true if the `tetromino` is inside the grid on empty slots.
    pub(in crate::app::player) fn is_tetromino_valid(&self, tetromino: &Tetromino) -> bool {
        for block in tetromino.blocks() {
            if self.is_block_available(block).is_err() {
                return false;
            }
        }
        true
    }

    /// Push the Tetromino into the grid and return the number of lines completed.
    pub(in crate::app::player) fn freeze_tetromino(
        &mut self,
        tetromino: &mut Tetromino,
    ) -> Option<u64> {
        let mut game_over = true;
        let mut blocks = tetromino.split();
        for block in &mut blocks {
            self[block] = Some(block.color());
            self.line_sum[block.y() as usize] += 1;
            // if there's a block below the top of the visible grid, continue playing
            if block.y() as usize > 1 {
                game_over = false;
            }
        }
        if game_over {
            return None;
        }
        let mut score = 0;
        for y in 0..self.nb_rows {
            if self.line_sum[y as usize] == self.nb_columns {
                // TODO neatly separate this in a private function
                self.matrix.remove(y as usize);
                self.matrix.insert(0, vec![None; self.nb_columns as usize]);

                self.line_sum.remove(y as usize);
                self.line_sum.insert(0, 0);
                score += 1;
            }
        }
        Some(score)
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
        let empty = rng.gen::<u32>() % self.nb_columns;

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

    /// Draw a 1 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn one(&mut self, tetromino: Tetromino) {
        let tetris_color = tetromino.blocks[0].color();
        self.null();
        // starting from the top of the number
        // careful, it's matrix[y][x] and y increases towards the bottom
        self.matrix[9][5] = Some(tetris_color);
        self.matrix[10][4] = Some(tetris_color);
        self.matrix[10][5] = Some(tetris_color);
        //? self.matrix[11][3] = Some(tetris_color);
        self.matrix[11][5] = Some(tetris_color);
        self.matrix[12][5] = Some(tetris_color);
        self.matrix[13][3] = Some(tetris_color);
        self.matrix[13][4] = Some(tetris_color);
        self.matrix[13][5] = Some(tetris_color);
        self.matrix[13][6] = Some(tetris_color);
    }

    /// Draw a 2 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn two(&mut self, tetromino: Tetromino) {
        let tetris_color = tetromino.blocks[0].color();
        self.null();
        // starting from the top of the number
        // careful, it's matrix[y][x] and y increases towards the bottom
        self.matrix[9][4] = Some(tetris_color);
        self.matrix[9][5] = Some(tetris_color);
        self.matrix[10][3] = Some(tetris_color);
        self.matrix[10][6] = Some(tetris_color);
        self.matrix[11][5] = Some(tetris_color);
        self.matrix[12][4] = Some(tetris_color);
        self.matrix[13][3] = Some(tetris_color);
        self.matrix[13][4] = Some(tetris_color);
        self.matrix[13][5] = Some(tetris_color);
        self.matrix[13][6] = Some(tetris_color);
    }

    /// Draw a 3 with blocks of the same color as tetromino.
    pub(in crate::app::player) fn three(&mut self, tetromino: Tetromino) {
        let tetris_color = tetromino.blocks[0].color();
        self.null();
        // starting from the top of the number
        // careful, it's matrix[y][x] and y increases towards the bottom
        self.matrix[9][4] = Some(tetris_color);
        self.matrix[9][5] = Some(tetris_color);
        self.matrix[10][3] = Some(tetris_color);
        self.matrix[10][6] = Some(tetris_color);
        self.matrix[11][5] = Some(tetris_color);
        self.matrix[12][3] = Some(tetris_color);
        self.matrix[12][6] = Some(tetris_color);
        self.matrix[13][4] = Some(tetris_color);
        self.matrix[13][5] = Some(tetris_color);
    }

    pub(in crate::app::player) fn total_width(&self) -> f64 {
        self.nb_columns as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn visible_height(&self) -> f64 {
        (self.nb_rows - self.nb_hidden_rows) as f64 * BLOCK_SIZE
    }

    pub(in crate::app::player) fn hidden_height(&self) -> f64 {
        self.nb_hidden_rows as f64 * BLOCK_SIZE
    }
}
