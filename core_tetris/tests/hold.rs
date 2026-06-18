use core_tetris::{
    BagType, LineClear, MockRng, TetrisCommand, TetrisPlayer, TetrisResult, TetrominoKind,
};

#[test]
fn hold_resets() -> TetrisResult {
    let rng =
        &mut MockRng::tetromino_cycle(&[TetrominoKind::I, TetrominoKind::T, TetrominoKind::T]);
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
    assert!(player.hold_queue().is_none());

    player.apply_player_move(TetrisCommand::Clockwise, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "     I   \n",
            "     I   \n",
            "---------\n",
            "     I   \n",
            "     I   \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
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
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
    player.apply_player_move(TetrisCommand::Clockwise, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "    TT   \n",
            "---------\n",
            "    T    \n",
            "         \n",
            "         \n",
            "         \n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

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
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::T));

    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );

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
            "   IIII  \n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::T));

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "   IIII  \n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    Ok(())
}

#[test]
fn cant_hold_twice() -> TetrisResult {
    let rng =
        &mut MockRng::tetromino_cycle(&[TetrominoKind::I, TetrominoKind::T, TetrominoKind::T]);
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
    assert!(player.hold_queue().is_none());

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
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
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
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
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

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
            "    T    \n",
            "   TTT   \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::T));

    Ok(())
}

#[test]
fn hold_remembers() -> TetrisResult {
    let start_rng = &mut MockRng::tetromino_cycle(&[
        TetrominoKind::I,
        TetrominoKind::T,
        TetrominoKind::T,
        TetrominoKind::T,
        TetrominoKind::T,
        TetrominoKind::T,
    ]);
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(start_rng, BagType::NoBag);

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
    assert!(player.hold_queue().is_none());

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
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
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::I));

    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
    while player.try_left() {}
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
    while player.try_right() {}
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::Single)
    );

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n",
            "   TTT   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            " T  T  T \n",
            "---------\n",
        )
    );

    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;

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
            " T  T  T \n",
            "---------\n",
        )
    );
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::T));

    Ok(())
}
