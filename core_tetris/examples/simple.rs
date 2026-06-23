use core_tetris::{
    BagType, LineClear, MockRng, TetrisCommand, TetrisPlayer, TetrisResult, TetrominoKind,
};

fn main() -> TetrisResult {
    // ANCHOR: tetris_player_creation
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n", // Buffer Zone
            "   TTT   \n", // the first tetromino was chosen according to `rng`
            "---------\n", // Skyline
            "         \n", // Matrix
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    // ANCHOR_END: tetris_player_creation
    // ANCHOR: tetris_player_basic_commands
    let garbage_rng = &mut MockRng::default(); // right-aligned garbage
    player.apply_player_move(TetrisCommand::Left, rng, garbage_rng)?;
    assert_eq!(player.last_line_clear(), LineClear::None);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "   T     \n",
            "  TTT    \n",
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
    player.apply_player_move(TetrisCommand::Clockwise, rng, garbage_rng)?;
    assert_eq!(player.last_line_clear(), LineClear::None);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "   T     \n",
            "   TT    \n",
            "---------\n",
            "   T     \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        )
    );
    player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng)?;
    assert_eq!(player.last_line_clear(), LineClear::None);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n", // the second tetromino was chosen according to `rng`
            "    OO   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "   T     \n",
            "   TT    \n",
            "   T     \n",
            "---------\n",
        )
    );
    // ANCHOR_END: tetris_player_basic_commands
    // ANCHOR: tetris_player_hold_command
    player.apply_player_move(TetrisCommand::Hold, rng, garbage_rng)?;
    assert_eq!(player.last_line_clear(), LineClear::None);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n", // the third tetromino was chosen according to `rng`
            "   TTT   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "   T     \n",
            "   TT    \n",
            "   T     \n",
            "---------\n",
        )
    );
    player.push_garbage(1);
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
            "   T     \n",
            "   TT    \n",
            "   T     \n",
            "---------\n",
        )
    );
    // ANCHOR_END: tetris_player_hold_command
    // ANCHOR: tetris_player_garbage
    player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng)?;
    assert_eq!(player.last_line_clear(), LineClear::None);
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            "    T    \n",
            "   TTT   \n",
            "   T     \n",
            "   TT    \n",
            "   T     \n",
            " XXXXXXXX\n", // garbage is right aligned as specified in `garbage_rng`
            "---------\n",
        )
    );
    assert_eq!(
        player.apply_player_move(TetrisCommand::HardDrop, rng, garbage_rng),
        Err(core_tetris::GameOverError::LockOut)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    TO   \n",
            "   TTT   \n", // the active tetromino is not in a valid state wrt the grid
            "---------\n",
            "    T    \n",
            "   TTT   \n",
            "   T     \n",
            "   TT    \n",
            "   T     \n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );
    // ANCHOR_END: tetris_player_garbage
    Ok(())
}
