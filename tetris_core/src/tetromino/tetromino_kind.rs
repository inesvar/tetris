//! Define `enum` [TetrominoKind].
use super::{Direction, Position, RotationType};
use serde::{Deserialize, Serialize};

/// Lists the 7 types of [Tetromino](super::Tetromino)s.
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TetrominoKind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl TetrominoKind {
    pub(crate) const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];

    /// Return the initial position of the center and the blocks.
    pub(super) fn get_initial_position(&self) -> [Position; 5] {
        // source: Tetris Guideline
        match self {
            // in order : center_x, center_y, first_block_x, first_block_y, second_block_x, second_block_y...
            TetrominoKind::I => [init(1, 1), init(0, 1), init(1, 1), init(2, 1), init(3, 1)],
            TetrominoKind::O => [init(1, 0), init(1, 0), init(2, 0), init(1, 1), init(2, 1)],
            TetrominoKind::T => [init(1, 1), init(1, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::J => [init(1, 1), init(0, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::L => [init(1, 1), init(2, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::S => [init(1, 1), init(1, 0), init(2, 0), init(0, 1), init(1, 1)],
            TetrominoKind::Z => [init(1, 1), init(0, 0), init(1, 0), init(1, 1), init(2, 1)],
        }
    }

    /// Return if the rotation center is on a block center.
    pub(super) fn is_rotation_center_on_block_center(&self) -> bool {
        *self != TetrominoKind::I && *self != TetrominoKind::O
    }

    /// Return wall-kick translations.
    pub(super) fn wall_kick_translations(
        &self,
        rtype: RotationType,
        rotation_status: Direction,
    ) -> impl Iterator<Item = &'static Position> {
        match self {
            TetrominoKind::O => NO_WALL_KICKS[..].iter(),
            TetrominoKind::I => Self::i_wall_kick_translations(rtype, rotation_status).iter(),
            TetrominoKind::T
                if rotation_status == Direction::North || rotation_status == Direction::South =>
            {
                // NOTE : T tetromino has some weird additional stuff (missing kicks)
                Self::generic_wall_kick_translations(rtype, rotation_status).iter()
            }
            _ => Self::generic_wall_kick_translations(rtype, rotation_status).iter(),
        }
    }
}

const fn init(x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ 0..4, y @ 0..4) => Position::new(x, y),
        _ => panic!("x and y should be between 0 and 4 excluded"),
    }
}

macro_rules! const_map {
    ($array:expr, $func:ident) => {
        [
            $array[0].$func(),
            $array[1].$func(),
            $array[2].$func(),
            $array[3].$func(),
            $array[4].$func(),
        ]
    };
}

macro_rules! const_swap_map {
    ($array:expr, $func:ident) => {
        [
            $array[0].$func(),
            $array[2].$func(),
            $array[1].$func(),
            $array[4].$func(),
            $array[3].$func(),
        ]
    };
}

// source: Tetris Guideline
const NORTH_TO_EAST_WALL_KICKS: [Position; 5] = [
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, -1),
    Position::new(0, 2),
    Position::new(-1, 2),
];

const NORTH_TO_WEST_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg_x);
const EAST_TO_SOUTH_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg);
const EAST_TO_NORTH_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg);
const SOUTH_TO_WEST_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg_x);
const SOUTH_TO_EAST_WALL_KICKS: [Position; 5] = NORTH_TO_EAST_WALL_KICKS;
const WEST_TO_NORTH_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg_y);
const WEST_TO_SOUTH_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_WALL_KICKS, neg_y);

const NORTH_TO_EAST_I_WALL_KICKS: [Position; 5] = [
    Position::new(0, 0),
    Position::new(-2, 0),
    Position::new(1, 0),
    Position::new(-2, 1),
    Position::new(1, -2),
];

const NORTH_TO_WEST_I_WALL_KICKS: [Position; 5] =
    const_swap_map!(NORTH_TO_EAST_I_WALL_KICKS, neg_x);
const EAST_TO_SOUTH_I_WALL_KICKS: [Position; 5] =
    const_swap_map!(NORTH_TO_EAST_I_WALL_KICKS, neg_x);
const EAST_TO_NORTH_I_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_I_WALL_KICKS, neg);
const SOUTH_TO_WEST_I_WALL_KICKS: [Position; 5] = const_map!(NORTH_TO_EAST_I_WALL_KICKS, neg);
const SOUTH_TO_EAST_I_WALL_KICKS: [Position; 5] =
    const_swap_map!(NORTH_TO_EAST_I_WALL_KICKS, neg_y);
const WEST_TO_NORTH_I_WALL_KICKS: [Position; 5] =
    const_swap_map!(NORTH_TO_EAST_I_WALL_KICKS, neg_y);
const WEST_TO_SOUTH_I_WALL_KICKS: [Position; 5] = NORTH_TO_EAST_I_WALL_KICKS;

const NO_WALL_KICKS: [Position; 1] = [Position::new(0, 0)];

impl TetrominoKind {
    fn i_wall_kick_translations(
        rtype: RotationType,
        rotation_status: Direction,
    ) -> &'static [Position] {
        match (rotation_status, rtype) {
            (Direction::North, RotationType::Clockwise) => &NORTH_TO_EAST_I_WALL_KICKS,
            (Direction::East, RotationType::Counterclockwise) => &EAST_TO_NORTH_I_WALL_KICKS,
            (Direction::East, RotationType::Clockwise) => &EAST_TO_SOUTH_I_WALL_KICKS,
            (Direction::South, RotationType::Counterclockwise) => &SOUTH_TO_EAST_I_WALL_KICKS,
            (Direction::South, RotationType::Clockwise) => &SOUTH_TO_WEST_I_WALL_KICKS,
            (Direction::West, RotationType::Counterclockwise) => &WEST_TO_SOUTH_I_WALL_KICKS,
            (Direction::West, RotationType::Clockwise) => &WEST_TO_NORTH_I_WALL_KICKS,
            (Direction::North, RotationType::Counterclockwise) => &NORTH_TO_WEST_I_WALL_KICKS,
            (_, _) => &NO_WALL_KICKS,
        }
    }
    fn generic_wall_kick_translations(
        rtype: RotationType,
        rotation_status: Direction,
    ) -> &'static [Position] {
        match (rotation_status, rtype) {
            (Direction::North, RotationType::Clockwise) => &NORTH_TO_EAST_WALL_KICKS,
            (Direction::East, RotationType::Counterclockwise) => &EAST_TO_NORTH_WALL_KICKS,
            (Direction::East, RotationType::Clockwise) => &EAST_TO_SOUTH_WALL_KICKS,
            (Direction::South, RotationType::Counterclockwise) => &SOUTH_TO_EAST_WALL_KICKS,
            (Direction::South, RotationType::Clockwise) => &SOUTH_TO_WEST_WALL_KICKS,
            (Direction::West, RotationType::Counterclockwise) => &WEST_TO_SOUTH_WALL_KICKS,
            (Direction::West, RotationType::Clockwise) => &WEST_TO_NORTH_WALL_KICKS,
            (Direction::North, RotationType::Counterclockwise) => &NORTH_TO_WEST_WALL_KICKS,
            (_, _) => &NO_WALL_KICKS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Position, RotationType, TetrominoKind};
    use rstest::rstest;

    impl Position {
        fn from_str(string: &str) -> (Position, Vec<Position>) {
            let lines: Vec<&str> = string.split('\n').collect();

            let mut positions = Vec::new();
            let mut center: Position = Position::default();

            for (row, line) in lines.iter().enumerate() {
                for (col, cell) in line.char_indices() {
                    match cell {
                        'C' => {
                            center = Position::new(col as i32, row as i32);
                            positions.push(center);
                        }
                        '#' => {
                            positions.push(Position::new(col as i32, row as i32));
                        }
                        _ => continue,
                    }
                }
            }

            (center, positions)
        }
    }

    macro_rules! positions {
        ($(($x:expr, $y:expr)),+ $(,)?) => {
            [$( Position::new($x, $y) ),+]
        };
    }

    #[rstest]
    // NOTE: according to the *Tetris Guideline*, all tetrominos centers
    // are at the same place. For the O tetromino, we make an exception
    // to ensure that even if the tetromino is rotated around the bottom
    // right corner of the center, it will not move.
    #[case::o(TetrominoKind::O, concat!(
            " C#      \n",
            " ##      \n",
        ))]
    #[case::i(TetrominoKind::I, concat!(
            "         \n",
            "#C##     \n",
        ))]
    #[case::t(TetrominoKind::T, concat!(
            " #       \n",
            "#C#      \n",
        ))]
    #[case::l(TetrominoKind::L, concat!(
            "  #      \n",
            "#C#      \n",
        ))]
    #[case::j(TetrominoKind::J, concat!(
            "#        \n",
            "#C#      \n",
        ))]
    #[case::s(TetrominoKind::S, concat!(
            " ##      \n",
            "#C       \n",
        ))]
    #[case::z(TetrominoKind::Z, concat!(
            "##       \n",
            " C#      \n",
        ))]
    fn get_initial_position_is_correct(#[case] kind: TetrominoKind, #[case] expected: &str) {
        let expected = Position::from_str(expected);
        let actual = kind.get_initial_position();

        assert_eq!(actual[0], expected.0);
        assert_eq!(actual[1..], expected.1);
    }

    #[test]
    fn generic_wall_kicks_are_correct() {
        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::North
            ),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::East
            ),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::East),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::South
            ),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::South
            ),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::West
            ),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::West),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::North
            ),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );
    }

    #[test]
    fn i_wall_kicks_are_correct() {
        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::North),
            positions!((0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::East
            ),
            positions!((0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::East),
            positions!((0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::South
            ),
            positions!((0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::South),
            positions!((0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::West
            ),
            positions!((0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::West),
            positions!((0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::North
            ),
            positions!((0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1))
        );
    }
}
