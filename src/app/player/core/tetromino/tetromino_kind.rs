//! Define `enum` [TetrominoKind].
use super::{Deserialize, Direction, Position, RotationType, Serialize, TetrisColor};

/// Seven types of Tetromino.
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub(in crate::app::player) enum TetrominoKind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
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

const UP_TO_RIGHT_WALL_KICKS: [Position; 5] = [
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(-1, 1),
    Position::new(0, -2),
    Position::new(-1, -2),
];

const RIGHT_TO_UP_WALL_KICKS: [Position; 5] = const_map!(UP_TO_RIGHT_WALL_KICKS, neg);
const DOWN_TO_LEFT_WALL_KICKS: [Position; 5] = const_map!(UP_TO_RIGHT_WALL_KICKS, mirror_x);
const LEFT_TO_DOWN_WALL_KICKS: [Position; 5] = const_map!(UP_TO_RIGHT_WALL_KICKS, mirror_y);

const UP_TO_RIGHT_I_WALL_KICKS: [Position; 5] = [
    Position::new(0, 0),
    Position::new(-2, 0),
    Position::new(1, 0),
    Position::new(-2, -1),
    Position::new(1, 2),
];

const RIGHT_TO_BOTTOM_I_WALL_KICKS: [Position; 5] = [
    Position::new(0, 0),
    Position::new(-1, 0),
    Position::new(2, 0),
    Position::new(-1, 2),
    Position::new(2, -1),
];

const RIGHT_TO_UP_I_WALL_KICKS: [Position; 5] = const_map!(UP_TO_RIGHT_I_WALL_KICKS, neg);
const BOTTOM_TO_RIGHT_I_WALL_KICKS: [Position; 5] = const_map!(RIGHT_TO_BOTTOM_I_WALL_KICKS, neg);

const NO_WALL_KICKS: [Position; 1] = [Position::new(0, 0)];

const fn init(x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ 0..4, y @ 0..4) => Position::new(x, y),
        _ => panic!("x and y should be between 0 and 4 excluded"),
    }
}

impl TetrominoKind {
    pub(in crate::app::player::core) const ALL: [Self; 7] = [
        Self::I,
        Self::J,
        Self::L,
        Self::O,
        Self::S,
        Self::T,
        Self::Z,
    ];

    /// Return the initial position of the center and the blocks.
    pub(super) fn get_initial_position(&self) -> [Position; 5] {
        // cf https://tetris.fandom.com/wiki/SRS#Spawn_Orientation_and_Location
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

    /// Return an array of the 5 SRS wall-kick translations.
    pub(super) fn wall_kick_translations(
        &self,
        rtype: RotationType,
        rotation_status: Direction,
    ) -> &'static [Position] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match self {
            // it's useless calling this function for the O tetromino, since it doesn't move when turned
            TetrominoKind::O => &NO_WALL_KICKS,
            TetrominoKind::I => Self::i_wall_kick_translations(rtype, rotation_status),
            _ => Self::generic_wall_kick_translations(rtype, rotation_status),
        }
    }

    fn i_wall_kick_translations(
        rtype: RotationType,
        rotation_status: Direction,
    ) -> &'static [Position] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match (rotation_status, rtype) {
            (Direction::North, RotationType::Clockwise) => &UP_TO_RIGHT_I_WALL_KICKS,
            (Direction::East, RotationType::Counterclockwise) => &RIGHT_TO_UP_I_WALL_KICKS,
            (Direction::East, RotationType::Clockwise) => &RIGHT_TO_BOTTOM_I_WALL_KICKS,
            (Direction::South, RotationType::Counterclockwise) => &BOTTOM_TO_RIGHT_I_WALL_KICKS,
            (Direction::South, RotationType::Clockwise) => &RIGHT_TO_UP_I_WALL_KICKS,
            (Direction::West, RotationType::Counterclockwise) => &UP_TO_RIGHT_I_WALL_KICKS,
            (Direction::West, RotationType::Clockwise) => &BOTTOM_TO_RIGHT_I_WALL_KICKS,
            (Direction::North, RotationType::Counterclockwise) => &RIGHT_TO_BOTTOM_I_WALL_KICKS,
            (_, _) => &NO_WALL_KICKS,
        }
    }

    fn generic_wall_kick_translations(
        rtype: RotationType,
        rotation_status: Direction,
    ) -> &'static [Position] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match (rotation_status, rtype) {
            (Direction::North, RotationType::Clockwise) => &UP_TO_RIGHT_WALL_KICKS,
            (Direction::East, RotationType::Counterclockwise) => &RIGHT_TO_UP_WALL_KICKS,
            (Direction::East, RotationType::Clockwise) => &RIGHT_TO_UP_WALL_KICKS,
            (Direction::South, RotationType::Counterclockwise) => &UP_TO_RIGHT_WALL_KICKS,
            (Direction::South, RotationType::Clockwise) => &DOWN_TO_LEFT_WALL_KICKS,
            (Direction::West, RotationType::Counterclockwise) => &LEFT_TO_DOWN_WALL_KICKS,
            (Direction::West, RotationType::Clockwise) => &LEFT_TO_DOWN_WALL_KICKS,
            (Direction::North, RotationType::Counterclockwise) => &DOWN_TO_LEFT_WALL_KICKS,
            (_, _) => &NO_WALL_KICKS,
        }
    }
}

impl From<TetrominoKind> for TetrisColor {
    fn from(kind: TetrominoKind) -> Self {
        match kind {
            TetrominoKind::I => TetrisColor::Cyan,
            TetrominoKind::O => TetrisColor::Yellow,
            TetrominoKind::T => TetrisColor::Purple,
            TetrominoKind::J => TetrisColor::Blue,
            TetrominoKind::L => TetrisColor::Orange,
            TetrominoKind::S => TetrisColor::Green,
            TetrominoKind::Z => TetrisColor::Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Position, RotationType, TetrominoKind};

    macro_rules! positions {
        ($(($x:expr, $y:expr)),+ $(,)?) => {
            [$( Position::new($x, $y) ),+]
        };
    }

    #[test]
    fn spawn_positions_are_correct() {
        assert_eq!(
            TetrominoKind::I.get_initial_position(),
            positions!((4, 1), (3, 1), (4, 1), (5, 1), (6, 1))
        );

        assert_eq!(
            TetrominoKind::O.get_initial_position(),
            positions!((5, 1), (4, 0), (5, 0), (4, 1), (5, 1))
        );
        assert_eq!(
            TetrominoKind::T.get_initial_position(),
            positions!((4, 1), (4, 0), (3, 1), (4, 1), (5, 1))
        );
        assert_eq!(
            TetrominoKind::J.get_initial_position(),
            positions!((4, 1), (3, 0), (3, 1), (4, 1), (5, 1))
        );
        assert_eq!(
            TetrominoKind::L.get_initial_position(),
            positions!((4, 1), (5, 0), (3, 1), (4, 1), (5, 1))
        );
        assert_eq!(
            TetrominoKind::S.get_initial_position(),
            positions!((4, 1), (4, 0), (5, 0), (3, 1), (4, 1))
        );
        assert_eq!(
            TetrominoKind::Z.get_initial_position(),
            positions!((4, 1), (3, 0), (4, 0), (4, 1), (5, 1))
        );
    }

    #[test]
    fn generic_wall_kicks_are_correct() {
        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::North
            ),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::East
            ),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::East),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::South
            ),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::South
            ),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::West
            ),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::West),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::North
            ),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );
    }

    #[test]
    fn i_wall_kicks_are_correct() {
        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::North),
            positions!((0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::East
            ),
            positions!((0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::East),
            positions!((0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::South
            ),
            positions!((0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::South),
            positions!((0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::West
            ),
            positions!((0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(RotationType::Clockwise, Direction::West),
            positions!((0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1))
        );

        assert_eq!(
            TetrominoKind::i_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::North
            ),
            positions!((0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1))
        );
    }
}
