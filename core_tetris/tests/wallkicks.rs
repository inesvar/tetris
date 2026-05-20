use core_tetris::{
    BagType, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult, TetrominoKind,
};
use rstest::rstest;
use std::str::FromStr;

#[rstest]
/// Testing that operations "moving to the right wall" and "turning 90°" are commutable,
/// in situations where the tetromino faces east or west (this encompasses all situations that
/// only work thanks to wall kicks).
fn off_the_right_wall(
    #[values(
        TetrominoKind::I,
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
    initial_rotation: TetrisCommand,
    #[values(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

    let mut no_wall_kick_player = player.clone();
    no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    while player.try_right() {}
    while no_wall_kick_player.try_right() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}

#[rstest]
/// Testing that operations "moving to the left wall" and "turning 90°" are commutable,
/// in situations where the tetromino faces east or west (this encompasses all situations that
/// only work thanks to wall kicks).
fn off_the_left_wall(
    #[values(
        TetrominoKind::I,
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
    initial_rotation: TetrisCommand,
    #[values(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

    let mut no_wall_kick_player = player.clone();
    no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    while player.try_left() {}
    while no_wall_kick_player.try_left() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}

#[rstest]
/// Testing that operations "moving to the floor" and "turning 90°" are commutable,
/// in situations where the tetromino faces north (this encompasses all situations that
/// only work thanks to wall kicks).
fn off_the_floor(
    #[values(
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)] rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    let mut no_wall_kick_player = player.clone();
    no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    match rotation {
        TetrisCommand::Clockwise => no_wall_kick_player.try_left(),
        TetrisCommand::Counterclockwise => no_wall_kick_player.try_right(),
        _ => unreachable!(),
    };

    while player.try_fall() {}
    while no_wall_kick_player.try_fall() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}

#[rstest]
#[case::east_to_north(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
#[case::west_to_north(TetrisCommand::Counterclockwise, TetrisCommand::Clockwise)]
fn out_of_right_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
    #[values(
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        "         \n",
        "         \n",
        "         \n",
        "XXXXXXX  \n",
        "XXXXXXX  \n",
        "XXXXXXX  \n",
        "---------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();
    let mut no_wall_kick_player = player.clone();

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

    while player.try_right() {}
    while no_wall_kick_player.try_fall() {}

    while player.try_fall() {}
    while no_wall_kick_player.try_right() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}

#[rstest]
#[case::east_to_north(TetrisCommand::Clockwise, TetrisCommand::Counterclockwise)]
#[case::west_to_north(TetrisCommand::Counterclockwise, TetrisCommand::Clockwise)]
fn out_of_left_well(
    #[case] initial_rotation: TetrisCommand,
    #[case] rotation: TetrisCommand,
    #[values(
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        "         \n",
        "         \n",
        "         \n",
        "  XXXXXXX\n",
        "  XXXXXXX\n",
        "  XXXXXXX\n",
        "---------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();
    let mut no_wall_kick_player = player.clone();

    player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

    while player.try_left() {}
    while no_wall_kick_player.try_fall() {}

    while player.try_fall() {}
    while no_wall_kick_player.try_left() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}
