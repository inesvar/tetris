//! Define `struct` [Tetromino], re-export `struct` [Position] and `enum`s [TetrominoKind] and [RotationType].
#![doc = simple_mermaid::mermaid!("tetromino/tetromino.mmd")]

mod moving_primitives;
mod rotation_translation;
mod spatial_primitives;
mod tetromino_kind;

use self::{
    moving_primitives::ApplyRotationTranslation, rotation_translation::RotationTranslation,
    spatial_primitives::Direction,
};
use super::{GameOverError, TetrisColor, TetrisGrid, TetrominoMove};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(super) use rotation_translation::RotationType;
pub(in crate::app::player) use spatial_primitives::Position;
pub(in crate::app::player) use tetromino_kind::TetrominoKind;

/// Tetromino.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub(in crate::app) struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    pub(super) blocks: [Position; 4],
    direction: Direction,
}

impl From<TetrominoKind> for Tetromino {
    fn from(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        Tetromino {
            kind,
            center: positions[0],
            blocks: [positions[1], positions[2], positions[3], positions[4]],
            direction: Direction::default(),
        }
    }
}

#[doc = simple_mermaid::mermaid!("tetromino/tetromino_internals.mmd")]
impl Tetromino {
    pub(in crate::app::player) fn new(kind: TetrominoKind) -> Self {
        Self::from(kind)
    }

    pub(in crate::app::player) fn reset(&mut self) {
        *self = Self::from(self.kind)
    }

    pub(in crate::app::player) fn can_enter_grid(
        &mut self,
        grid: &TetrisGrid,
    ) -> Result<(), GameOverError> {
        let offset = grid.can_blocks_spawn_on(&self.blocks)?;
        let translation = RotationTranslation::translation(offset);

        self.translate(&translation);
        Ok(())
    }

    pub(in crate::app::player) fn lock_down(
        self,
        grid: &mut TetrisGrid,
    ) -> Result<u64, GameOverError> {
        grid.add_blocks_and_clear_lines(&self.blocks, self.color())
    }

    pub(in crate::app::player) fn apply(
        &mut self,
        tetromino_move: TetrominoMove,
        grid: &TetrisGrid,
    ) -> bool {
        if tetromino_move.get_rotation_type() == RotationType::Identity {
            self.apply_translation(tetromino_move, grid)
        } else {
            self.apply_rotation(tetromino_move, grid)
        }
    }

    fn apply_translation(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> bool {
        let translation = RotationTranslation::translation(tetromino_move.get_translation());
        let moved = self.try_move(grid, &translation);

        if tetromino_move.is_repeated() {
            while self.try_move(grid, &translation) {}
        }

        moved
    }

    fn apply_rotation(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> bool {
        let rotation_type = tetromino_move.get_rotation_type();

        let wall_kicks_translations =
            TetrominoKind::wall_kick_translations(&self.kind, rotation_type, self.direction);
        for wall_kick in wall_kicks_translations {
            let movement = RotationTranslation::new(wall_kick, rotation_type, &self.center);
            if self.try_move(grid, &movement) {
                return true;
            }
        }

        false
    }

    /// Return whether the tetromino could be moved.
    fn try_move(&mut self, grid: &TetrisGrid, movement: &RotationTranslation) -> bool {
        let mut new_blocks = self.blocks;
        for new_block in new_blocks.iter_mut() {
            if self.kind.is_rotation_center_on_block_center() {
                new_block.move_by(movement);
            } else {
                new_block.move_by_with_offset(movement);
            }
            // Check if `copy` is inside `grid` and on an empty slot.
            if !grid.is_block_available(new_block) {
                return false;
            }
        }
        self.blocks = new_blocks;
        self.direction.move_by(movement);
        self.center.translate_by(movement);
        true
    }

    /// Translate the tetromino.
    fn translate(&mut self, movement: &RotationTranslation) {
        for block in self.blocks.iter_mut() {
            block.translate_by(movement);
        }
        self.center.translate_by(movement);
    }

    pub(in crate::app::player) fn blocks(&self) -> &[Position] {
        &self.blocks
    }

    pub(in crate::app::player) fn color(&self) -> TetrisColor {
        self.kind.into()
    }
}

// TODO remove this...
/// `Tetromino::default` is far from the origin and is not renderer on the screen.
impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            kind: TetrominoKind::O,
            center: Position::default(),
            blocks: [Position::new(-50, -50); 4],
            direction: Direction::default(),
        }
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: Vec<char> = crate::concatln!("         ", "         ", "         ", "         ",)
            .chars()
            .collect();

        for block in self.blocks {
            buffer[10 * block.y() as usize + block.x() as usize] = '#';
        }
        buffer[10 * self.center.y() as usize + self.center.x() as usize] = 'C';

        let buffer = String::from_iter(buffer);

        write!(f, "{buffer}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concatln;
    use rstest::rstest;

    #[rstest]
    #[case::o_north(TetrominoKind::O, Direction::North, concatln!(
            " C#      ",
            " ##      ",
            "         ",
            "         ",
        ))]
    #[case::o_east(TetrominoKind::O, Direction::East, concatln!(
            " C#      ",
            " ##      ",
            "         ",
            "         ",
        ))]
    #[case::o_south(TetrominoKind::O, Direction::South, concatln!(
            " C#      ",
            " ##      ",
            "         ",
            "         ",
        ))]
    #[case::o_west(TetrominoKind::O, Direction::West, concatln!(
            " C#      ",
            " ##      ",
            "         ",
            "         ",
        ))]
    #[case::i_north(TetrominoKind::I, Direction::North, concatln!(
            "         ",
            "#C##     ",
            "         ",
            "         ",
        ))]
    #[case::i_east(TetrominoKind::I, Direction::East, concatln!(
            "  #      ",
            " C#      ",
            "  #      ",
            "  #      ",
        ))]
    #[case::i_south(TetrominoKind::I, Direction::South, concatln!(
            "         ",
            " C       ",
            "####     ",
            "         ",
        ))]
    #[case::i_west(TetrominoKind::I, Direction::West, concatln!(
            " #       ",
            " C       ",
            " #       ",
            " #       ",
        ))]
    #[case::l_north(TetrominoKind::L, Direction::North, concatln!(
            "  #      ",
            "#C#      ",
            "         ",
            "         ",
        ))]
    #[case::l_east(TetrominoKind::L, Direction::East, concatln!(
            " #       ",
            " C       ",
            " ##      ",
            "         ",
        ))]
    #[case::l_south(TetrominoKind::L, Direction::South, concatln!(
            "         ",
            "#C#      ",
            "#        ",
            "         ",
        ))]
    #[case::l_west(TetrominoKind::L, Direction::West, concatln!(
            "##       ",
            " C       ",
            " #       ",
            "         ",
        ))]
    #[case::j_north(TetrominoKind::J, Direction::North, concatln!(
            "#        ",
            "#C#      ",
            "         ",
            "         ",
        ))]
    #[case::j_east(TetrominoKind::J, Direction::East, concatln!(
            " ##      ",
            " C       ",
            " #       ",
            "         ",
        ))]
    #[case::j_south(TetrominoKind::J, Direction::South, concatln!(
            "         ",
            "#C#      ",
            "  #      ",
            "         ",
        ))]
    #[case::j_west(TetrominoKind::J, Direction::West, concatln!(
            " #       ",
            " C       ",
            "##       ",
            "         ",
        ))]
    #[case::s_north(TetrominoKind::S, Direction::North, concatln!(
            " ##      ",
            "#C       ",
            "         ",
            "         ",
        ))]
    #[case::s_east(TetrominoKind::S, Direction::East, concatln!(
            " #       ",
            " C#      ",
            "  #      ",
            "         ",
        ))]
    #[case::s_south(TetrominoKind::S, Direction::South, concatln!(
            "         ",
            " C#      ",
            "##       ",
            "         ",
        ))]
    #[case::s_west(TetrominoKind::S, Direction::West, concatln!(
            "#        ",
            "#C       ",
            " #       ",
            "         ",
        ))]
    #[case::z_north(TetrominoKind::Z, Direction::North, concatln!(
            "##       ",
            " C#      ",
            "         ",
            "         ",
        ))]
    #[case::z_east(TetrominoKind::Z, Direction::East, concatln!(
            "  #      ",
            " C#      ",
            " #       ",
            "         ",
        ))]
    #[case::z_south(TetrominoKind::Z, Direction::South, concatln!(
            "         ",
            "#C       ",
            " ##      ",
            "         ",
        ))]
    #[case::z_west(TetrominoKind::Z, Direction::West, concatln!(
            " #       ",
            "#C       ",
            "#        ",
            "         ",
        ))]
    fn tetromino_rotation_is_correct(
        #[case] kind: TetrominoKind,
        #[case] direction: Direction,
        #[case] expected_blocks: &str,
    ) {
        let mut tetromino = Tetromino::new(kind);
        let empty_grid = TetrisGrid::default();

        match direction {
            Direction::North => {}
            Direction::East => {
                tetromino.apply(TetrominoMove::Clockwise, &empty_grid);
            }
            Direction::South => {
                tetromino.apply(TetrominoMove::HalfTurn, &empty_grid);
            }
            Direction::West => {
                tetromino.apply(TetrominoMove::Counterclockwise, &empty_grid);
            }
        }

        assert_eq!(tetromino.to_string(), expected_blocks, "{}", tetromino);
        assert_eq!(tetromino.direction, direction);
    }
}
