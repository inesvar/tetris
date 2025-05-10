//! Define the specificities of the 7 kinds of Tetromino pieces : colors, starting positions, wall-kicks.
use super::{Direction, Position, RotationType, TetrisColor, TetrominoKind};

impl TetrominoKind {
    /// Returns the name of the TetrominoKind variant.
    pub(super) fn get(&self) -> String {
        match self {
            TetrominoKind::I => "I".to_owned(),
            TetrominoKind::O => "O".to_owned(),
            TetrominoKind::T => "T".to_owned(),
            TetrominoKind::J => "J".to_owned(),
            TetrominoKind::L => "L".to_owned(),
            TetrominoKind::S => "S".to_owned(),
            TetrominoKind::Z => "Z".to_owned(),
        }
    }

    /// Returns the initial position of the center and the blocks of a tetromino.
    pub(super) fn get_initial_position(&self) -> [i32; 10] {
        // cf https://tetris.fandom.com/wiki/SRS#Spawn_Orientation_and_Location
        match self {
            // in order : center_x, center_y, first_block_x, first_block_y, second_block_x, second_block_y...
            TetrominoKind::I => [4, 1, 3, 1, 4, 1, 5, 1, 6, 1],
            TetrominoKind::O => [5, 1, 4, 0, 4, 1, 5, 0, 5, 1],
            TetrominoKind::T => [4, 1, 5, 1, 3, 1, 4, 1, 4, 0],
            TetrominoKind::J => [4, 1, 3, 0, 3, 1, 4, 1, 5, 1],
            TetrominoKind::L => [4, 1, 3, 1, 4, 1, 5, 1, 5, 0],
            TetrominoKind::S => [4, 1, 3, 1, 4, 1, 4, 0, 5, 0],
            TetrominoKind::Z => [4, 1, 3, 0, 4, 0, 4, 1, 5, 1],
        }
    }

    /// Returns the color associated with the TetrominoKind.
    pub(super) fn get_color(&self) -> TetrisColor {
        match self {
            TetrominoKind::I => TetrisColor::Cyan,
            TetrominoKind::O => TetrisColor::Yellow,
            TetrominoKind::T => TetrisColor::Purple,
            TetrominoKind::J => TetrisColor::Blue,
            TetrominoKind::L => TetrisColor::Orange,
            TetrominoKind::S => TetrisColor::Green,
            TetrominoKind::Z => TetrisColor::Red,
        }
    }

    /// Returns an array of the 5 SRS wall-kick translations.
    pub(super) fn wall_kicks_translations(
        &self,
        rtype: RotationType,
        rotation_status: Direction,
    ) -> [Position; 5] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match self {
            // since the O piece doesn't even rotate
            TetrominoKind::O => unreachable!(),
            // the additional calculation for the I piece is to compensate for the fact
            // that its rotational center is actually between two blocks
            // and not on a block like in the code
            // (the true position of the initial center is (4.5, 1) and not (4, 1) see line 32.)
            // (its due to the fact that the I piece doesn't have a 3x3 bounding box like the other rotating pieces)
            TetrominoKind::I => match (rotation_status, rtype) {
                (Direction::Up, RotationType::Clockwise) => [
                    Position::new(1, 0),
                    Position::new(1, 0).add(Position::new(-2, 0)),
                    Position::new(1, 0).add(Position::new(1, 0)),
                    Position::new(1, 0).add(Position::new(-2, -1)),
                    Position::new(1, 0).add(Position::new(1, 2)),
                ],
                (Direction::Right, RotationType::Counterclockwise) => [
                    Position::new(-1, 0),
                    Position::new(-1, 0).add(Position::new(2, 0)),
                    Position::new(-1, 0).add(Position::new(-1, 0)),
                    Position::new(-1, 0).add(Position::new(2, 1)),
                    Position::new(-1, 0).add(Position::new(-1, -2)),
                ],
                (Direction::Right, RotationType::Clockwise) => [
                    Position::new(0, 1),
                    Position::new(0, 1).add(Position::new(-1, 0)),
                    Position::new(0, 1).add(Position::new(2, 0)),
                    Position::new(0, 1).add(Position::new(-1, 2)),
                    Position::new(0, 1).add(Position::new(2, -1)),
                ],
                (Direction::Down, RotationType::Counterclockwise) => [
                    Position::new(0, -1),
                    Position::new(0, -1).add(Position::new(1, 0)),
                    Position::new(0, -1).add(Position::new(-2, 0)),
                    Position::new(0, -1).add(Position::new(1, -2)),
                    Position::new(0, -1).add(Position::new(-2, 1)),
                ],
                (Direction::Down, RotationType::Clockwise) => [
                    Position::new(-1, 0),
                    Position::new(-1, 0).add(Position::new(2, 0)),
                    Position::new(-1, 0).add(Position::new(-1, 0)),
                    Position::new(-1, 0).add(Position::new(2, 1)),
                    Position::new(-1, 0).add(Position::new(-1, -2)),
                ],
                (Direction::Left, RotationType::Counterclockwise) => [
                    Position::new(1, 0),
                    Position::new(1, 0).add(Position::new(-2, 0)),
                    Position::new(1, 0).add(Position::new(1, 0)),
                    Position::new(1, 0).add(Position::new(-2, -1)),
                    Position::new(1, 0).add(Position::new(1, 2)),
                ],
                (Direction::Left, RotationType::Clockwise) => [
                    Position::new(0, -1),
                    Position::new(0, -1).add(Position::new(1, 0)),
                    Position::new(0, -1).add(Position::new(-2, 0)),
                    Position::new(0, -1).add(Position::new(1, -2)),
                    Position::new(0, -1).add(Position::new(-2, 1)),
                ],
                (Direction::Up, RotationType::Counterclockwise) => [
                    Position::new(0, 1),
                    Position::new(0, 1).add(Position::new(-1, 0)),
                    Position::new(0, 1).add(Position::new(2, 0)),
                    Position::new(0, 1).add(Position::new(-1, 2)),
                    Position::new(0, 1).add(Position::new(2, -1)),
                ],
                (_, _) => todo!(),
            },
            _ => match (rotation_status, rtype) {
                (Direction::Up, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(-1, 1),
                    Position::new(0, -2),
                    Position::new(-1, -2),
                ],
                (Direction::Right, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, -1),
                    Position::new(0, 2),
                    Position::new(1, 2),
                ],
                (Direction::Right, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, -1),
                    Position::new(0, 2),
                    Position::new(1, 2),
                ],
                (Direction::Down, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(-1, 1),
                    Position::new(0, -2),
                    Position::new(-1, -2),
                ],
                (Direction::Down, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(0, -2),
                    Position::new(1, -2),
                ],
                (Direction::Left, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(-1, -1),
                    Position::new(0, 2),
                    Position::new(-1, 2),
                ],
                (Direction::Left, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(-1, -1),
                    Position::new(0, 2),
                    Position::new(-1, 2),
                ],
                (Direction::Up, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(0, -2),
                    Position::new(1, -2),
                ],
                (_, _) => todo!(),
            },
        }
    }
}
