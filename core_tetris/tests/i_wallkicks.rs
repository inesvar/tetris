use std::str::FromStr;

use core_tetris::{
    BagType, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult, TetrominoKind, TetrominoMove
};
use rstest::rstest;

#[rstest]
#[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Clockwise.into())]
#[case::west_to_south(TetrominoMove::Counterclockwise.into(), TetrominoMove::Counterclockwise.into())]
fn i_off_the_right_wall_to_the_bottom(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::garbage_cycle(&[], 0);
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_right() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "        I\n",
            "        I\n",
            "---------\n",
            "        I\n",
            "        I\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "     IIII\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::east_to_north(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
#[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Clockwise.into())]
fn i_off_the_right_wall_to_the_top(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::garbage_cycle(&[], 0);
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_right() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "        I\n",
            "        I\n",
            "---------\n",
            "        I\n",
            "        I\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "     IIII\n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Clockwise.into())]
#[case::west_to_south(TetrominoMove::Counterclockwise.into(), TetrominoMove::Counterclockwise.into())]
fn i_off_the_left_wall_to_the_bottom(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_left() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "I        \n",
            "I        \n",
            "---------\n",
            "I        \n",
            "I        \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "IIII     \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::east_to_north(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
#[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Clockwise.into())]
fn i_off_the_left_wall_to_the_top(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_left() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "I        \n",
            "I        \n",
            "---------\n",
            "I        \n",
            "I        \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "IIII     \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::north_to_west(None, TetrominoMove::Counterclockwise.into())]
#[case::south_to_east(Some(TetrominoMove::HalfTurn.into()), TetrominoMove::Counterclockwise.into())]
fn i_off_the_floor_to_the_left(
    #[case] initial_rotation: Option<TetrisCommand>,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    if let Some(command) = initial_rotation {
        player.try_apply(command, &mut rng, &mut garbage_rng)?;
    }
    while player.try_fall() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "   IIII  \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "   I     \n",
            "   I     \n",
            "   I     \n",
            "   I     \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::north_to_east(None, TetrominoMove::Clockwise.into())]
#[case::south_to_west(Some(TetrominoMove::HalfTurn.into()), TetrominoMove::Clockwise.into())]
fn i_off_the_floor_to_the_right(
    #[case] initial_rotation: Option<TetrisCommand>,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    if let Some(command) = initial_rotation {
        player.try_apply(command, &mut rng, &mut garbage_rng)?;
    }
    while player.try_fall() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "   IIII  \n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "      I  \n",
            "      I  \n",
            "      I  \n",
            "      I  \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Clockwise.into())]
#[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Clockwise.into())]
fn i_out_of_right_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )).unwrap();
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_right() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "        I\n",
            "        I\n",
            "---------\n",
            "        I\n",
            "        I\n",
            "         \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );

    assert!(player.try_fall());
    assert!(player.try_fall());
    assert!(player.try_fall());
    assert!(player.try_fall());

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "        I\n",
            "XXXXXXXXI\n",
            "XXXXXXXXI\n",
            "XXXXXXXXI\n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "     IIII\n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );

    Ok(())
}

#[rstest]
#[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
#[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Counterclockwise.into())]
fn i_out_of_left_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )).unwrap();
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "   IIII  \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
    while player.try_left() {}

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "I        \n",
            "I        \n",
            "---------\n",
            "I        \n",
            "I        \n",
            "         \n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert!(player.try_fall());
    assert!(player.try_fall());
    assert!(player.try_fall());
    assert!(player.try_fall());

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "I        \n",
            "IXXXXXXXX\n",
            "IXXXXXXXX\n",
            "IXXXXXXXX\n",
            "---------\n",
        )
    );

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "         \n",
            "         \n",
            "---------\n",
            "         \n",
            "         \n",
            "IIII     \n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    Ok(())
}