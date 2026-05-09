//! Define `struct` [TetrisGrid] and `enum` [GameOverError].
#![doc = simple_mermaid::mermaid!("tetris_grid.mmd")]

mod tetris_color;

use super::Position;
use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
pub use tetris_color::TetrisColor;

// TODO: fix UI when the value is different from 2.
/// Number of visible lines above the **Skyline**.
pub const NB_VISIBLE_BUFFER_ROWS: u32 = 2;

/// Tetris grid.
///
/// According to the **Tetris Guideline**, the grid has 2 components:
/// - **Matrix**: "the rectangular arrangement of cells creating the active game area, usually 10 columns wide by 20 rows high.
///   Tetriminos fall from the top-middle just above the **Skyline** (off-screen) to the bottom."
/// - **Buffer Zone**: "a 10-cell wide x 20-cell high invisible area above the Matrix used to detect **Lock
///   Out**, **Block Out**, and **Top Out** **Game Over Conditions**."
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TetrisGrid {
    nb_columns: i32,
    nb_matrix_rows: i32,
    nb_buffer_rows: i32,
    /// **Matrix** and **Buffer Zone** cells, indexed *from bottom to top*.
    cells: Vec<Vec<Option<TetrisColor>>>,
    /// Number of filled blocks in each line of the [TetrisGrid::cells].
    line_sum: Vec<i32>,
}

/// `Ok(())` on success, [GameOverError] on fail.
pub type TetrisResult = Result<(), GameOverError>;

// same visibility as TetrisColor
/// **Tetris Guideline** Game Over Conditions.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum GameOverError {
    /// According to the **Tetris Guideline** :
    ///
    /// "[...] occurs when part of a newly-generated tetrimino is blocked due to
    /// an existing Block in the **Matrix**."
    BlockOut,
    /// According to the **Tetris Guideline** :
    ///
    /// "[...] occurs when a whole tetrimino locks down above the **Skyline**."
    LockOut,
    /// According to the **Tetris Guideline** :
    ///
    /// "[...] occurs when an opponent’s **Line Attack** forces your Blocks past the top
    /// of the 20-line **Buffer zone**. It is highly unlikely that this will ever occur,
    /// since **Lock out** [...] or **Block out** [...] will likely occur before a Block ever
    /// gets pushed out of the **Buffer zone**."
    TopOut,
}

impl Display for GameOverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockOut => write!(f, "can not spawn tetromino in the Matrix"),
            Self::LockOut => write!(f, "tetromino locked down above the Skyline"),
            Self::TopOut => write!(f, "a Block was pushed past the top of the Buffer Zone"),
        }
    }
}

impl Error for GameOverError {}

/// Constructors.
#[allow(missing_docs)]
impl TetrisGrid {
    /// Creates an empty grid of the specified size. Use [TetrisGrid::default] for **Tetris Guideline** official size.
    ///
    /// # Panics
    ///
    /// If `nb_columns` or `nb_matrix_rows` or `nb_buffer_rows` aren't in the expected range.
    ///
    /// `nb_columns` has to be between [TetrisGrid::MINIMAL_NB_COLUMNS (4)](TetrisGrid::MINIMAL_NB_COLUMNS) and [TetrisGrid::MAX (100)](TetrisGrid::MAX).
    /// Should be [TetrisGrid::DEFAULT_NB_COLUMNS (10)](TetrisGrid::DEFAULT_NB_COLUMNS) according to the **Tetris Guideline**.
    ///
    /// `nb_matrix_rows` has to be between [TetrisGrid::MINIMAL_NB_MATRIX_ROWS (6)](TetrisGrid::MINIMAL_NB_MATRIX_ROWS) and [TetrisGrid::MAX (100)](TetrisGrid::MAX).
    /// Should be [TetrisGrid::DEFAULT_NB_MATRIX_ROWS (20)](TetrisGrid::DEFAULT_NB_MATRIX_ROWS) according to the **Tetris Guideline**.
    ///
    /// `nb_buffer_rows` has to be between [TetrisGrid::MINIMAL_NB_BUFFER_ROWS (2)](TetrisGrid::MINIMAL_NB_BUFFER_ROWS) and [TetrisGrid::MAX (100)](TetrisGrid::MAX).
    /// Should be [TetrisGrid::DEFAULT_NB_BUFFER_ROWS (20)](TetrisGrid::DEFAULT_NB_BUFFER_ROWS) according to the **Tetris Guideline**.
    pub fn new(nb_columns: u32, nb_matrix_rows: u32, nb_buffer_rows: u32) -> Self {
        if nb_columns > TetrisGrid::MAX
            || nb_matrix_rows > TetrisGrid::MAX
            || nb_buffer_rows > TetrisGrid::MAX
        {
            panic!("`nb_columns`, `nb_matrix_rows` and `nb_buffer_rows` should be less than `TetrisGrid::MAX`");
        }
        if nb_columns < TetrisGrid::MINIMAL_NB_COLUMNS {
            panic!(
                "`nb_columns` should be greater than or equal to `TetrisGrid::MINIMAL_NB_COLUMNS`"
            );
        }
        if nb_matrix_rows < TetrisGrid::MINIMAL_NB_MATRIX_ROWS {
            panic!("`nb_matrix_rows` should be greater than or equal to `TetrisGrid::MINIMAL_NB_MATRIX_ROWS`");
        }
        if nb_buffer_rows < TetrisGrid::MINIMAL_NB_BUFFER_ROWS {
            panic!("`nb_buffer_rows` should be greater than or equal to `TetrisGrid::MINIMAL_NB_BUFFER_ROWS`");
        }

        let nb_rows_usize = (nb_matrix_rows + nb_buffer_rows) as usize;
        let mut matrix = Vec::with_capacity(nb_rows_usize);
        for _ in 0..nb_rows_usize {
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
            line_sum: vec![0; nb_rows_usize],
        }
    }

    /// Creates a smaller [TetrisGrid] which is nice for printing.
    pub fn compact() -> Self {
        Self::new(9, 6, 2)
    }

    /// Creates the smallest possible [TetrisGrid].
    pub fn minimal() -> Self {
        Self::new(
            TetrisGrid::MINIMAL_NB_COLUMNS,
            TetrisGrid::MINIMAL_NB_MATRIX_ROWS,
            TetrisGrid::MINIMAL_NB_BUFFER_ROWS,
        )
    }

    /// A reasonable maximum for [TetrisGrid] dimensions.
    pub const MAX: u32 = 100;

    pub const DEFAULT_NB_COLUMNS: u32 = 10;
    pub const DEFAULT_NB_MATRIX_ROWS: u32 = 20;
    pub const DEFAULT_NB_BUFFER_ROWS: u32 = 20;

    pub const MINIMAL_NB_COLUMNS: u32 = 4;
    pub const MINIMAL_NB_MATRIX_ROWS: u32 = 6;
    pub const MINIMAL_NB_BUFFER_ROWS: u32 = 2;
}

impl TetrisGrid {
    /// Add garbage lines at the bottom of the grid depending on the number of completed lines.
    pub(crate) fn apply_received_garbage<R: Rng>(
        &mut self,
        nb_garbage_lines: u64,
        rng: &mut R,
    ) -> TetrisResult {
        for _ in 0..nb_garbage_lines {
            let empty = rng.random_range(0..self.nb_columns);
            self.add_garbage_row(empty as usize)?;
        }
        Ok(())
    }
}

/// Methods used  by [Tetromino](super::tetromino::Tetromino) to enter, move and then lock down in the grid.
impl TetrisGrid {
    /// Return the translation needed for `blocks` to enter the grid.
    pub(crate) fn get_starting_position(&self) -> Position {
        Position::new((self.nb_columns + 1) / 2 - 2, 0)
    }
}

#[doc = simple_mermaid::mermaid!("tetris_grid_internals.mmd")]
mod tetris_grid_internals {
    use super::*;

    impl TetrisGrid {
        /// Return true if the `block` is inside the grid in an empty slot.
        pub(crate) fn is_block_available(&self, block: &Position) -> bool {
            self.is_in_grid(block) && self.is_block_empty(block)
        }

        /// Push the blocks into the grid and return the number of lines completed.
        ///
        /// # Panics
        ///
        /// If any of the `blocks` is outside the tetris grid or not empty.
        pub(crate) fn add_blocks_and_clear_lines(
            &mut self,
            blocks: &[Position],
            tetris_color: TetrisColor,
        ) -> Result<u64, GameOverError> {
            let all_above_skyline = blocks.iter().all(|block| self.is_above_skyline(block));

            for block in blocks {
                self.add_block(block, tetris_color);
            }

            // Only continue playing if there's a block below the skyline
            if all_above_skyline {
                return Err(GameOverError::LockOut);
            }

            Ok(self.clear_lines())
        }

        /// Return whether `block` is available in the tetris grid.
        ///
        /// # Panics
        ///
        /// If `block` is outside the tetris grid.
        pub(super) fn is_block_empty(&self, block: &Position) -> bool {
            self[block].is_none()
        }

        /// Return `true` is `block` is in the **Matrix** or the **Buffer Zone**.
        pub(super) fn is_in_grid(&self, block: &Position) -> bool {
            let line = self.convert_position_y_to_grid_y(block.y());
            block.x() >= 0
                && line >= 0
                && block.x() < self.nb_columns
                && line < self.nb_matrix_rows + self.nb_buffer_rows
        }

        /// Remove complete lines, return number of cleared lines.
        fn clear_lines(&mut self) -> u64 {
            let mut score = 0;
            for y in (0..self.nb_rows_usize()).rev() {
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
        /// If `row` is greater or equal to `self.cells.len()`.
        pub(super) fn pop_row(&mut self, row: usize) {
            self.cells.remove(row);
            self.line_sum.remove(row);

            self.cells.insert(
                self.nb_rows_usize() - 1,
                vec![None; self.nb_columns as usize],
            );
            self.line_sum.insert(self.nb_rows_usize() - 1, 0);
        }

        /// Add garbage row to the tetris grid (only if the top row is empty).
        ///
        /// # Panics
        ///
        /// If `empty` is greater or equal to `self.nb_columns`, or `self.line_sum` is empty.
        pub(super) fn add_garbage_row(&mut self, empty: usize) -> TetrisResult {
            if *self.line_sum.last().unwrap() > 0 {
                return Err(GameOverError::TopOut);
            }
            self.line_sum.insert(0, self.nb_columns - 1);
            self.cells
                .insert(0, vec![Some(TetrisColor::Grey); self.nb_columns as usize]);
            self.cells[0][empty] = None;

            self.line_sum.remove(self.nb_rows_usize());
            self.cells.remove(self.nb_rows_usize());
            Ok(())
        }

        /// Add `block` to the tetris grid.
        ///
        /// # Panics
        ///
        /// If `block` is outside the tetris grid or not empty.
        pub(super) fn add_block(&mut self, block: &Position, tetris_color: TetrisColor) {
            if !self.is_block_empty(block) {
                panic!("Tried adding a block to a non-empty cell")
            }
            let line = self.convert_position_y_to_grid_y(block.y()) as usize;
            self.cells[line][block.x() as usize] = Some(tetris_color);
            self.line_sum[line] += 1;
        }

        fn is_above_skyline(&self, block: &Position) -> bool {
            self.convert_position_y_to_grid_y(block.y()) >= self.nb_matrix_rows
        }

        const fn nb_rows_usize(&self) -> usize {
            (self.nb_matrix_rows + self.nb_buffer_rows) as usize
        }
    }
}

/// Getters.
#[allow(missing_docs)]
impl TetrisGrid {
    pub const fn nb_matrix_rows_i32(&self) -> i32 {
        self.nb_matrix_rows
    }

    pub const fn nb_columns_i32(&self) -> i32 {
        self.nb_columns
    }

    /// Iterator over all the grid cells (**Matrix** and **Buffer Zone**).
    pub fn positions(&self) -> impl Iterator<Item = Position> + use<'_> {
        let h = self.nb_matrix_rows + self.nb_buffer_rows;
        let w = self.nb_columns;
        (0..h).flat_map(move |y| {
            (0..w).map(move |x| Position::new(x, self.convert_grid_y_to_position_y(y)))
        })
    }
}

impl TetrisGrid {
    fn convert_position_y_to_grid_y(&self, y: i32) -> i32 {
        -y + self.nb_matrix_rows + NB_VISIBLE_BUFFER_ROWS as i32 - 1
    }

    fn convert_grid_y_to_position_y(&self, y: i32) -> i32 {
        -y + self.nb_matrix_rows + NB_VISIBLE_BUFFER_ROWS as i32 - 1
    }
}

impl Index<&Position> for TetrisGrid {
    type Output = Option<TetrisColor>;

    fn index(&self, block: &Position) -> &<Self as Index<&Position>>::Output {
        let line = self.convert_position_y_to_grid_y(block.y()) as usize;
        &self.cells[line][block.x() as usize]
    }
}

impl IndexMut<&Position> for TetrisGrid {
    fn index_mut(&mut self, block: &Position) -> &mut <Self as Index<&Position>>::Output {
        let line = self.convert_position_y_to_grid_y(block.y()) as usize;
        &mut self.cells[line][block.x() as usize]
    }
}

/// Tetris grid according to the **Tetris Guideline**.
impl Default for TetrisGrid {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_NB_COLUMNS,
            Self::DEFAULT_NB_MATRIX_ROWS,
            Self::DEFAULT_NB_BUFFER_ROWS,
        )
    }
}

impl Display for TetrisGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = "-".repeat(self.nb_columns as usize);
        writeln!(f, "{separator}")?;
        for (index, line) in self.cells.iter().enumerate().rev() {
            for cell in line {
                if let Some(tetris_color) = cell {
                    write!(f, "{tetris_color}")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;

            if self.nb_matrix_rows == index as i32 {
                writeln!(f, "{separator}")?;
            }
        }
        writeln!(f, "{separator}")?;
        Ok(())
    }
}

impl FromStr for TetrisGrid {
    type Err = String;

    /// # Panics
    ///
    /// See [TetrisGrid::new].
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = string.split_terminator('\n').collect();

        let nb_columns = lines[0].len();
        if !lines.iter().all(|line| line.len() == nb_columns) {
            return Err(format!(
                "Lines should have equal length.\nlines: {:?}",
                lines
            ));
        }

        let mut iter = lines.split(|line| line.chars().all(|char| char == '-'));

        let Some(buffer) = iter.nth(1) else {
            return Err(format!(
                "There should be 3 lines of '-'.\nlines: {:?}",
                lines
            ));
        };
        let Some(matrix) = iter.next() else {
            return Err(format!(
                "There should be 3 lines of '-'.\nlines: {:?}",
                lines
            ));
        };

        let nb_matrix_rows = matrix.len();
        let nb_buffer_rows = buffer.len();
        let mut tetris_grid = TetrisGrid::new(
            nb_columns as u32,
            nb_matrix_rows as u32,
            nb_buffer_rows as u32,
        );

        let cells = [buffer, matrix].concat();

        for (row, line) in cells.iter().rev().enumerate() {
            let pos_y = tetris_grid.convert_grid_y_to_position_y(row as i32);

            for (pos_x, cell) in line.char_indices() {
                let pos = Position::new(pos_x as i32, pos_y);

                if let Ok(tetris_color) = cell.try_into() {
                    tetris_grid.add_block(&pos, tetris_color);
                }
            }
        }

        Ok(tetris_grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn tetris_grid_max_is_safe_to_cast() {
        assert!(i32::try_from(TetrisGrid::MAX).is_ok());
        assert!(usize::try_from(TetrisGrid::MAX).is_ok());
    }

    #[rstest]
    #[case(TetrisGrid::MAX + 1, TetrisGrid::DEFAULT_NB_MATRIX_ROWS, TetrisGrid::DEFAULT_NB_BUFFER_ROWS)]
    #[case(TetrisGrid::DEFAULT_NB_COLUMNS, TetrisGrid::MAX + 1, TetrisGrid::DEFAULT_NB_BUFFER_ROWS)]
    #[case(TetrisGrid::DEFAULT_NB_COLUMNS, TetrisGrid::DEFAULT_NB_MATRIX_ROWS, TetrisGrid::MAX + 1)]
    #[should_panic(
        expected = "`nb_columns`, `nb_matrix_rows` and `nb_buffer_rows` should be less than `TetrisGrid::MAX`"
    )]
    fn new_panics_if_any_arg_is_too_big(
        #[case] nb_columns: u32,
        #[case] nb_matrix_rows: u32,
        #[case] nb_buffer_rows: u32,
    ) {
        TetrisGrid::new(nb_columns, nb_matrix_rows, nb_buffer_rows);
    }

    #[rstest]
    #[should_panic(
        expected = "`nb_columns` should be greater than or equal to `TetrisGrid::MINIMAL_NB_COLUMNS`"
    )]
    #[case(
        TetrisGrid::MINIMAL_NB_COLUMNS - 1,
        TetrisGrid::DEFAULT_NB_MATRIX_ROWS,
        TetrisGrid::DEFAULT_NB_BUFFER_ROWS
    )]
    #[should_panic(
        expected = "`nb_matrix_rows` should be greater than or equal to `TetrisGrid::MINIMAL_NB_MATRIX_ROWS`"
    )]
    #[case(TetrisGrid::DEFAULT_NB_COLUMNS, TetrisGrid::MINIMAL_NB_MATRIX_ROWS - 1, TetrisGrid::DEFAULT_NB_BUFFER_ROWS)]
    #[should_panic(
        expected = "`nb_buffer_rows` should be greater than or equal to `TetrisGrid::MINIMAL_NB_BUFFER_ROWS`"
    )]
    #[case(TetrisGrid::DEFAULT_NB_COLUMNS, TetrisGrid::DEFAULT_NB_MATRIX_ROWS, TetrisGrid::MINIMAL_NB_BUFFER_ROWS - 1)]
    fn new_panics_if_any_arg_is_too_small(
        #[case] nb_columns: u32,
        #[case] nb_matrix_rows: u32,
        #[case] nb_buffer_rows: u32,
    ) {
        TetrisGrid::new(nb_columns, nb_matrix_rows, nb_buffer_rows);
    }

    #[rstest]
    #[case::default(TetrisGrid::default())]
    #[case::compact(TetrisGrid::compact())]
    #[case::smallest_possible_grid(TetrisGrid::minimal())]
    #[case::biggest_possible_grid(TetrisGrid::new(
        TetrisGrid::MAX,
        TetrisGrid::MAX,
        TetrisGrid::MAX
    ))]
    fn new_doesnt_panic_if_all_args_are_correct(#[case] _grid: TetrisGrid) {}

    #[rstest]
    #[case::default(
        TetrisGrid::DEFAULT_NB_COLUMNS,
        TetrisGrid::DEFAULT_NB_MATRIX_ROWS,
        TetrisGrid::DEFAULT_NB_BUFFER_ROWS
    )]
    #[case::biggest_possible(TetrisGrid::MAX, TetrisGrid::MAX, TetrisGrid::MAX)]
    fn new_is_correct(
        #[case] nb_columns: u32,
        #[case] nb_matrix_rows: u32,
        #[case] nb_buffer_rows: u32,
    ) {
        let grid = TetrisGrid::new(nb_columns, nb_matrix_rows, nb_buffer_rows);

        assert_eq!(grid.nb_columns, nb_columns as i32);
        assert_eq!(grid.nb_matrix_rows, nb_matrix_rows as i32);
        assert_eq!(grid.nb_buffer_rows, nb_buffer_rows as i32);
    }

    #[fixture]
    fn empty_compact_grid() -> String {
        TetrisGrid::compact().to_string()
    }

    #[fixture]
    fn empty_default_grid() -> String {
        TetrisGrid::default().to_string()
    }

    #[rstest]
    #[case::empty_compact_grid(&empty_compact_grid())]
    #[case::empty_default_grid(&empty_default_grid())]
    #[case::large_grid(concat!(
            "--------------\n",
            "              \n",
            "              \n",
            "--------------\n",
            "              \n",
            "              \n",
            "              \n",
            "    XX        \n",
            " X  XXXXX     \n",
            "XXXXXXXXX     \n",
            "--------------\n",
        ))]
    fn from_str_is_successful_when_input_well_formed(#[case] grid: &str) {
        assert!(TetrisGrid::from_str(grid).is_ok());
    }

    #[test]
    fn add_garbage_is_correct() {
        let mut grid = TetrisGrid::from_str(concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "        X\n",
            "X        \n",
            "XX       \n",
            "XXX      \n",
            "---------\n",
        ))
        .unwrap();

        assert!(grid.add_garbage_row(0).is_ok());
        assert!(grid.add_garbage_row(1).is_ok());
        assert!(grid.add_garbage_row(3).is_ok());

        assert_eq!(
            grid.to_string(),
            concat!(
                "---------\n",
                "         \n",
                "        X\n",
                "---------\n",
                "X        \n",
                "XX       \n",
                "XXX      \n",
                " XXXXXXXX\n",
                "X XXXXXXX\n",
                "XXX XXXXX\n",
                "---------\n",
            ),
            "Actual grid:\n{}",
            grid
        );
    }

    #[test]
    fn add_garbage_returns_error_on_top_out() {
        let mut grid = TetrisGrid::from_str(concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "        X\n",
            "        X\n",
            "        X\n",
            "        X\n",
            "        X\n",
            "        X\n",
            "---------\n",
        ))
        .unwrap();

        assert!(grid.add_garbage_row(0).is_ok());
        assert!(grid.add_garbage_row(0).is_ok());
        assert_eq!(
            grid.add_garbage_row(0),
            Err(GameOverError::TopOut),
            "Actual grid:\n{}",
            grid
        );
    }

    #[rstest]
    #[case(TetrisGrid::default(), 0, 20 + NB_VISIBLE_BUFFER_ROWS as i32 - 1)]
    #[case(TetrisGrid::default(), NB_VISIBLE_BUFFER_ROWS as i32, 20 - 1)]
    fn convert_position_y_to_grid_y_is_correct(
        #[case] grid: TetrisGrid,
        #[case] input: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(grid.convert_position_y_to_grid_y(input), expected);
    }

    #[rstest]
    #[case(TetrisGrid::default(), 20 - 1, NB_VISIBLE_BUFFER_ROWS as i32)]
    #[case(TetrisGrid::default(), 20 + NB_VISIBLE_BUFFER_ROWS as i32 - 1, 0)]
    fn convert_grid_y_to_position_y_is_correct(
        #[case] grid: TetrisGrid,
        #[case] input: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(grid.convert_grid_y_to_position_y(input), expected);
    }

    #[test]
    fn is_block_empty_is_correct() {
        let grid = TetrisGrid::from_str(concat!(
            "---------\n",
            "        X\n",
            "X       X\n",
            "---------\n",
            "X        \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "X       X\n",
            "---------\n",
        ))
        .unwrap();

        assert!(grid.is_block_empty(&Position::new(0, 0)));
        assert!(!grid.is_block_empty(&Position::new(8, 0)));
        assert!(!grid.is_block_empty(&Position::new(0, 1)));
        assert!(!grid.is_block_empty(&Position::new(8, 1)));

        assert!(!grid.is_block_empty(&Position::new(0, 2)));
        assert!(grid.is_block_empty(&Position::new(8, 2)));
        assert!(!grid.is_block_empty(&Position::new(0, 7)));
        assert!(!grid.is_block_empty(&Position::new(8, 7)));
    }

    #[rstest]
    #[case(TetrisGrid::default(), Position::new(-1, -18))]
    #[case(TetrisGrid::default(), Position::new(0, -19))]
    #[case(TetrisGrid::default(), Position::new(10, 21))]
    #[case(TetrisGrid::default(), Position::new(9, 22))]
    #[should_panic]
    fn is_block_empty_panics_outside_of_the_grid(#[case] grid: TetrisGrid, #[case] pos: Position) {
        grid.is_block_empty(&pos);
    }

    #[rstest]
    #[case(TetrisGrid::default(), Position::new(0, -18), Position::new(9, 21))]
    #[case(TetrisGrid::minimal(), Position::new(0, 0), Position::new(3, 7))]
    fn is_in_grid_is_correct(
        #[case] grid: TetrisGrid,
        #[case] top_left: Position,
        #[case] bottom_right: Position,
    ) {
        assert!(grid.is_in_grid(&top_left));
        assert!(grid.is_in_grid(&bottom_right));

        // neighbors of the grid corners that are outside of the grid
        // horizontal offset :
        assert!(!grid.is_in_grid(&(top_left + Position::new(-1, 0))));
        assert!(!grid.is_in_grid(&(bottom_right + Position::new(1, 0))));
        // vertical offset :
        assert!(!grid.is_in_grid(&(top_left + Position::new(0, -1))));
        assert!(!grid.is_in_grid(&(bottom_right + Position::new(0, 1))));
    }

    #[rstest]
    #[case(TetrisGrid::default(), Position::new(-1, -18))]
    #[case(TetrisGrid::default(), Position::new(0, -19))]
    #[case(TetrisGrid::default(), Position::new(10, 21))]
    #[case(TetrisGrid::default(), Position::new(9, 22))]
    fn is_block_available_returns_false_outside_of_the_grid(
        #[case] grid: TetrisGrid,
        #[case] outside_pos: Position,
    ) {
        assert!(!grid.is_block_available(&outside_pos));
    }

    #[test]
    fn pop_row_is_correct() {
        let mut tetris_grid = TetrisGrid::from_str(concat!(
            "-------------\n",
            "             \n",
            "             \n",
            "-------------\n",
            "             \n",
            "             \n",
            "XX XX X X X X\n",
            " X X X XX XX \n",
            "X X XX XX X X\n",
            "XXXXXXXXXXXXX\n",
            "-------------\n",
        ))
        .unwrap();

        tetris_grid.pop_row(3);

        assert_eq!(
            tetris_grid.to_string(),
            concat!(
                "-------------\n",
                "             \n",
                "             \n",
                "-------------\n",
                "             \n",
                "             \n",
                "             \n",
                " X X X XX XX \n",
                "X X XX XX X X\n",
                "XXXXXXXXXXXXX\n",
                "-------------\n",
            ),
            "Actual grid:\n{}",
            tetris_grid
        );
    }
}
