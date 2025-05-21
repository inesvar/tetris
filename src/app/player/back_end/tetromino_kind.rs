//! Implement [TetrominoKind].
use super::{Direction, Position, RotationType, TetrisColor, TetrominoKind};

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

const fn init(x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ 0..4, y @ 0..4) => Position::new(3 + x, y),
        _ => panic!("x and y should be between 0 and 4 excluded"),
    }
}

impl TetrominoKind {
    /// Return the initial position of the center and the blocks.
    pub(super) fn get_initial_position(&self) -> [Position; 5] {
        // cf https://tetris.fandom.com/wiki/SRS#Spawn_Orientation_and_Location
        match self {
            // in order : center_x, center_y, first_block_x, first_block_y, second_block_x, second_block_y...
            TetrominoKind::I => [init(1, 1), init(0, 1), init(1, 1), init(2, 1), init(3, 1)],
            TetrominoKind::O => [init(2, 1), init(1, 0), init(2, 0), init(1, 1), init(2, 1)],
            TetrominoKind::T => [init(1, 1), init(1, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::J => [init(1, 1), init(0, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::L => [init(1, 1), init(2, 0), init(0, 1), init(1, 1), init(2, 1)],
            TetrominoKind::S => [init(1, 1), init(1, 0), init(2, 0), init(0, 1), init(1, 1)],
            TetrominoKind::Z => [init(1, 1), init(0, 0), init(1, 0), init(1, 1), init(2, 1)],
        }
    }

    /// Return the associated color.
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

    /// Return if the rotation center is on a block center.
    pub(super) fn is_rotation_center_on_block_center(&self) -> bool {
        *self != TetrominoKind::I
    }

    // the rotation of the I and O should be tweaked, for better clarity. hummm yes but it's practical...
    // BUT before changing everything, unit tests need to be written ! yay !
    /// Return an array of the 5 SRS wall-kick translations.
    pub(super) fn wall_kick_translations(
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
                    Position::new(0, 0),
                    Position::new(-2, 0),
                    Position::new(1, 0),
                    Position::new(-2, -1),
                    Position::new(1, 2),
                ],
                (Direction::Right, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(2, 0),
                    Position::new(-1, 0),
                    Position::new(2, 1),
                    Position::new(-1, -2), // neg neg
                ],
                (Direction::Right, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(2, 0),
                    Position::new(-1, 2),
                    Position::new(2, -1),
                ],
                (Direction::Down, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(-2, 0),
                    Position::new(1, -2),
                    Position::new(-2, 1),
                ],
                (Direction::Down, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(2, 0),
                    Position::new(-1, 0),
                    Position::new(2, 1),
                    Position::new(-1, -2),
                ],
                (Direction::Left, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(-2, 0),
                    Position::new(1, 0),
                    Position::new(-2, -1),
                    Position::new(1, 2),
                ],
                (Direction::Left, RotationType::Clockwise) => [
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(-2, 0),
                    Position::new(1, -2),
                    Position::new(-2, 1),
                ],
                (Direction::Up, RotationType::Counterclockwise) => [
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(2, 0),
                    Position::new(-1, 2),
                    Position::new(2, -1),
                ],
                (_, _) => todo!(),
            },
            _ => Self::generic_wall_kick_translations(rtype, rotation_status),
        }
    }

    fn generic_wall_kick_translations(
        rtype: RotationType,
        rotation_status: Direction,
    ) -> [Position; 5] {
        // cf https://tetris.fandom.com/wiki/SRS#Wall_Kicks
        match (rotation_status, rtype) {
            (Direction::Up, RotationType::Clockwise) => UP_TO_RIGHT_WALL_KICKS,
            (Direction::Right, RotationType::Counterclockwise) => RIGHT_TO_UP_WALL_KICKS,
            (Direction::Right, RotationType::Clockwise) => RIGHT_TO_UP_WALL_KICKS,
            (Direction::Down, RotationType::Counterclockwise) => UP_TO_RIGHT_WALL_KICKS,
            (Direction::Down, RotationType::Clockwise) => DOWN_TO_LEFT_WALL_KICKS,
            (Direction::Left, RotationType::Counterclockwise) => LEFT_TO_DOWN_WALL_KICKS,
            (Direction::Left, RotationType::Clockwise) => LEFT_TO_DOWN_WALL_KICKS,
            (Direction::Up, RotationType::Counterclockwise) => DOWN_TO_LEFT_WALL_KICKS,
            (_, _) => todo!(),
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
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Up),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Right
            ),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::Right
            ),
            positions!((0, 0), (1, 0), (1, -1), (0, 2), (1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Down
            ),
            positions!((0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Down),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Left
            ),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Left),
            positions!((0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2))
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Up
            ),
            positions!((0, 0), (1, 0), (1, 1), (0, -2), (1, -2))
        );
    }
}
