use core_tetris::{
    BagType, LineClear, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult,
    TetrominoKind,
};
use std::str::FromStr;

#[test]
fn line_clear_from_setup() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        "         \n",
        "         \n",
        "XXXXXXXX \n",
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
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );
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
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );
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
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 0);
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::Tetris)
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
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 4);
    assert_eq!(player.score(), 808);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 808);

    Ok(())
}

#[test]
fn manual_line_clear() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::O, TetrominoKind::I]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
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
    player.apply_player_move(TetrisCommand::Left, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "   OO    \n",
            "   OO    \n",
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
            "         \n",
            "   OO    \n",
            "   OO    \n",
            "---------\n",
        )
    );
    assert_eq!(player.score(), 12);
    while player.try_right() {}
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
            "   OO    \n",
            "   OO    \n",
            "---------\n",
        )
    );
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "   OO    \n",
            "   OOIIII\n",
            "---------\n",
        )
    );
    assert_eq!(player.score(), 24);
    while player.try_left() {}
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "OO       \n",
            "OO       \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "   OO    \n",
            "   OOIIII\n",
            "---------\n",
        )
    );
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
            "         \n",
            "OO OO    \n",
            "OO OOIIII\n",
            "---------\n",
        )
    );
    assert_eq!(player.score(), 36);
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
            "OO OO    \n",
            "OO OOIIII\n",
            "---------\n",
        )
    );
    player.apply_player_move(TetrisCommand::Left, rng, garbage_rng)?;
    player.apply_player_move(TetrisCommand::Left, rng, garbage_rng)?;
    player.apply_player_move(TetrisCommand::Left, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "  I      \n",
            "  I      \n",
            "---------\n",
            "  I      \n",
            "  I      \n",
            "         \n",
            "         \n",
            "OO OO    \n",
            "OO OOIIII\n",
            "---------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::Single)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "  I      \n",
            "  I      \n",
            "OOIOO    \n",
            "---------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 1);
    assert_eq!(player.score(), 144);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 144);

    Ok(())
}
