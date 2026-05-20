use core_tetris::{BagType, MockRng, TetrisCommand, TetrisPlayer, TetrisResult, TetrominoKind};

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

    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;

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

    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;

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

    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;

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
            "         \n",
            "         \n",
            "---------\n",
        )
    );

    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;

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
    assert!(player
        .hold_queue()
        .as_ref()
        .is_some_and(|tetromino| tetromino.kind() == TetrominoKind::T));

    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;

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

    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;

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

    player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng)?;
    while player.try_left() {}
    player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng)?;
    while player.try_right() {}
    player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng)?;

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

    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;

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
