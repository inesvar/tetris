//! Implement [TetrominoKind].
use super::{
    spatial_primitives::{FALL, LEFT, RIGHT, RISE},
    Direction, Position, RotationType, TetrisColor, TetrominoKind,
};

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

    // the rotation of the I and O that should be tweaked, for better clarity. hummm yes but it's practical...
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
                    RIGHT,
                    RIGHT.add(Position::new(-2, 0)),
                    RIGHT.add(Position::new(1, 0)),
                    RIGHT.add(Position::new(-2, -1)),
                    RIGHT.add(Position::new(1, 2)),
                ],
                (Direction::Right, RotationType::Counterclockwise) => [
                    LEFT,
                    LEFT.add(Position::new(2, 0)),
                    LEFT.add(Position::new(-1, 0)),
                    LEFT.add(Position::new(2, 1)),
                    LEFT.add(Position::new(-1, -2)), // neg neg
                ],
                (Direction::Right, RotationType::Clockwise) => [
                    FALL,
                    FALL.add(Position::new(-1, 0)),
                    FALL.add(Position::new(2, 0)),
                    FALL.add(Position::new(-1, 2)),
                    FALL.add(Position::new(2, -1)),
                ],
                (Direction::Down, RotationType::Counterclockwise) => [
                    RISE,
                    RISE.add(Position::new(1, 0)),
                    RISE.add(Position::new(-2, 0)),
                    RISE.add(Position::new(1, -2)),
                    RISE.add(Position::new(-2, 1)),
                ],
                (Direction::Down, RotationType::Clockwise) => [
                    LEFT,
                    LEFT.add(Position::new(2, 0)),
                    LEFT.add(Position::new(-1, 0)),
                    LEFT.add(Position::new(2, 1)),
                    LEFT.add(Position::new(-1, -2)),
                ],
                (Direction::Left, RotationType::Counterclockwise) => [
                    RIGHT,
                    RIGHT.add(Position::new(-2, 0)),
                    RIGHT.add(Position::new(1, 0)),
                    RIGHT.add(Position::new(-2, -1)),
                    RIGHT.add(Position::new(1, 2)),
                ],
                (Direction::Left, RotationType::Clockwise) => [
                    RISE,
                    RISE.add(Position::new(1, 0)),
                    RISE.add(Position::new(-2, 0)),
                    RISE.add(Position::new(1, -2)),
                    RISE.add(Position::new(-2, 1)),
                ],
                (Direction::Up, RotationType::Counterclockwise) => [
                    FALL,
                    FALL.add(Position::new(-1, 0)),
                    FALL.add(Position::new(2, 0)),
                    FALL.add(Position::new(-1, 2)),
                    FALL.add(Position::new(2, -1)),
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
                Position::new(1, 2), // neg
            ],
            (Direction::Right, RotationType::Clockwise) => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, -1),
                Position::new(0, 2),
                Position::new(1, 2), // neg
            ],
            (Direction::Down, RotationType::Counterclockwise) => [
                Position::new(0, 0),
                Position::new(-1, 0),
                Position::new(-1, 1),
                Position::new(0, -2),
                Position::new(-1, -2), // none
            ],
            (Direction::Down, RotationType::Clockwise) => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(0, -2),
                Position::new(1, -2), // none
            ],
            (Direction::Left, RotationType::Counterclockwise) => [
                Position::new(0, 0),
                Position::new(-1, 0),
                Position::new(-1, -1),
                Position::new(0, 2),
                Position::new(-1, 2), // neg
            ],
            (Direction::Left, RotationType::Clockwise) => [
                Position::new(0, 0),
                Position::new(-1, 0),
                Position::new(-1, -1),
                Position::new(0, 2),
                Position::new(-1, 2), // neg
            ],
            (Direction::Up, RotationType::Counterclockwise) => [
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(0, -2),
                Position::new(1, -2), // none
            ],
            (_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Position, RotationType, TetrominoKind};

    #[test]
    fn spawn_positions_are_correct() {
        assert_eq!(
            TetrominoKind::I.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(3, 1),
                Position::new(4, 1),
                Position::new(5, 1),
                Position::new(6, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::O.get_initial_position(),
            [
                Position::new(5, 1),
                Position::new(4, 0),
                Position::new(5, 0),
                Position::new(4, 1),
                Position::new(5, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::T.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(4, 0),
                Position::new(3, 1),
                Position::new(4, 1),
                Position::new(5, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::J.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(3, 0),
                Position::new(3, 1),
                Position::new(4, 1),
                Position::new(5, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::L.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(5, 0),
                Position::new(3, 1),
                Position::new(4, 1),
                Position::new(5, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::S.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(4, 0),
                Position::new(5, 0),
                Position::new(3, 1),
                Position::new(4, 1),
            ]
        );
        assert_eq!(
            TetrominoKind::Z.get_initial_position(),
            [
                Position::new(4, 1),
                Position::new(3, 0),
                Position::new(4, 0),
                Position::new(4, 1),
                Position::new(5, 1),
            ]
        );
    }

    const UP_TO_RIGHT_WALL_KICKS: [Position; 5] = [
        Position::new(0, 0),
        Position::new(-1, 0),
        Position::new(-1, 1),
        Position::new(0, -2),
        Position::new(-1, -2),
    ];

    const RIGHT_TO_UP_WALL_KICKS: [Position; 5] = [
        UP_TO_RIGHT_WALL_KICKS[0].neg(),
        UP_TO_RIGHT_WALL_KICKS[1].neg(),
        UP_TO_RIGHT_WALL_KICKS[2].neg(),
        UP_TO_RIGHT_WALL_KICKS[3].neg(),
        UP_TO_RIGHT_WALL_KICKS[4].neg(),
    ];

    const DOWN_TO_LEFT_WALL_KICKS: [Position; 5] = [
        UP_TO_RIGHT_WALL_KICKS[0].mirror_x(),
        UP_TO_RIGHT_WALL_KICKS[1].mirror_x(),
        UP_TO_RIGHT_WALL_KICKS[2].mirror_x(),
        UP_TO_RIGHT_WALL_KICKS[3].mirror_x(),
        UP_TO_RIGHT_WALL_KICKS[4].mirror_x(),
    ];

    const LEFT_TO_DOWN_WALL_KICKS: [Position; 5] = [
        UP_TO_RIGHT_WALL_KICKS[0].mirror_y(),
        UP_TO_RIGHT_WALL_KICKS[1].mirror_y(),
        UP_TO_RIGHT_WALL_KICKS[2].mirror_y(),
        UP_TO_RIGHT_WALL_KICKS[3].mirror_y(),
        UP_TO_RIGHT_WALL_KICKS[4].mirror_y(),
    ];

    #[test]
    fn generic_wall_kicks_are_correct() {
        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Up),
            UP_TO_RIGHT_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Right
            ),
            RIGHT_TO_UP_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Clockwise,
                Direction::Right
            ),
            RIGHT_TO_UP_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Down
            ),
            UP_TO_RIGHT_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Down),
            DOWN_TO_LEFT_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Left
            ),
            LEFT_TO_DOWN_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(RotationType::Clockwise, Direction::Left),
            LEFT_TO_DOWN_WALL_KICKS,
        );

        assert_eq!(
            TetrominoKind::generic_wall_kick_translations(
                RotationType::Counterclockwise,
                Direction::Up
            ),
            DOWN_TO_LEFT_WALL_KICKS,
        );
    }
}
