use core_tetris::{
    BagType, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult, TetrominoKind,
};
use rstest::rstest;
use std::str::FromStr;

#[rstest]
#[case::east_to_south(TetrisCommand::Clockwise, TetrisCommand::Clockwise)]
#[case::west_to_south(TetrisCommand::Counterclockwise, TetrisCommand::Counterclockwise)]
fn i_off_the_right_wall_to_the_bottom(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::garbage_cycle(&[], 0);
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
    player.try_apply(initial_rotation, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::east_to_north(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
#[case::west_to_north(TetrisCommand::Counterclockwise, TetrisCommand::Clockwise)]
fn i_off_the_right_wall_to_the_top(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::garbage_cycle(&[], 0);
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
    player.try_apply(initial_rotation, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::east_to_south(TetrisCommand::Clockwise, TetrisCommand::Clockwise)]
#[case::west_to_south(TetrisCommand::Counterclockwise, TetrisCommand::Counterclockwise)]
fn i_off_the_left_wall_to_the_bottom(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
    player.try_apply(initial_rotation, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::east_to_north(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
#[case::west_to_north(TetrisCommand::Counterclockwise, TetrisCommand::Clockwise)]
fn i_off_the_left_wall_to_the_top(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
    player.try_apply(initial_rotation, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::north_to_west(None, TetrisCommand::Counterclockwise)]
#[case::south_to_east(Some(TetrisCommand::HalfTurn), TetrisCommand::Counterclockwise)]
fn i_off_the_floor_to_the_left(
    #[case] initial_rotation: Option<TetrisCommand>,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
        player.try_apply(command, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::north_to_east(None, TetrisCommand::Clockwise)]
#[case::south_to_west(Some(TetrisCommand::HalfTurn), TetrisCommand::Clockwise)]
fn i_off_the_floor_to_the_right(
    #[case] initial_rotation: Option<TetrisCommand>,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

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
        player.try_apply(command, rng, garbage_rng)?;
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

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::east_to_south(TetrisCommand::Clockwise, TetrisCommand::Clockwise)]
#[case::west_to_north(TetrisCommand::Counterclockwise, TetrisCommand::Clockwise)]
fn i_out_of_right_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
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
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

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

    player.try_apply(initial_rotation, rng, garbage_rng)?;
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
            "        I\n",
            "XXXXXXXXI\n",
            "XXXXXXXXI\n",
            "XXXXXXXXI\n",
            "---------\n",
        )
    );

    player.try_apply(rotation, rng, garbage_rng)?;

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
#[case::east_to_north(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
#[case::west_to_south(TetrisCommand::Counterclockwise, TetrisCommand::Counterclockwise)]
fn i_out_of_left_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
) -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
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
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

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

    player.try_apply(initial_rotation, rng, garbage_rng)?;
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
            "I        \n",
            "IXXXXXXXX\n",
            "IXXXXXXXX\n",
            "IXXXXXXXX\n",
            "---------\n",
        )
    );

    player.try_apply(rotation, rng, garbage_rng)?;

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
