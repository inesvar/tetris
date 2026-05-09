use core_tetris::{
    BagType, MockRng, TetrisCommand, TetrisPlayer, TetrisResult, TetrominoKind, TetrominoMove,
};
use rstest::rstest;

#[rstest]
/// Testing that operations "moving to the right wall" and "turning 90°" are commutable,
/// in situations where the tetromino faces east or west (this encompasses all situations that
/// only work thanks to wall kicks).
fn off_the_right_wall(
    #[values(
        TetrominoKind::O,
        TetrominoKind::I,
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
    initial_rotation: TetrisCommand,
    #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
    rotation: TetrisCommand,
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
        TetrominoKind::O,
        TetrominoKind::I,
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
    initial_rotation: TetrisCommand,
    #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
    rotation: TetrisCommand,
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
        TetrominoKind::O,
        TetrominoKind::T,
        TetrominoKind::L,
        TetrominoKind::J,
        TetrominoKind::S,
        TetrominoKind::Z
    )]
    kind: TetrominoKind,
    #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
    rotation: TetrisCommand,
) -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[kind]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

    let mut no_wall_kick_player = player.clone();
    no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    if kind != TetrominoKind::O {
        match rotation {
            TetrisCommand::Move(TetrominoMove::Clockwise) => no_wall_kick_player.try_left(),
            TetrisCommand::Move(TetrominoMove::Counterclockwise) => no_wall_kick_player.try_right(),
            _ => unreachable!(),
        };
    }

    while player.try_fall() {}
    while no_wall_kick_player.try_fall() {}

    player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

    assert_eq!(player.to_string(), no_wall_kick_player.to_string());

    Ok(())
}
