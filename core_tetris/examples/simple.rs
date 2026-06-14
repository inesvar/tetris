use core_tetris::{
    BagType, LineClear, MockRng, TetrisCommand, TetrisPlayer, TetrisResult, TetrominoKind,
};

fn main() -> TetrisResult {
    // ANCHOR: tetris_player_creation
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
    let garbage_rng = &mut MockRng::default();
    let mut player = TetrisPlayer::compact(rng, BagType::NoBag);
    // ANCHOR_END: tetris_player_creation
    // ANCHOR: tetris_player_basic_commands
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n", // Buffer Zone
            "   TTT   \n", // the first tetromino is a T, as specified by `rng`
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
    player.try_apply(TetrisCommand::Left, rng, garbage_rng)?;
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
    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;
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
    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n", // the second tetromino is an O, as specified by `rng`
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
    player.try_apply(TetrisCommand::Hold, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    T    \n", // a new tetromino is automatically addded to the grid
            "   TTT   \n", // it's a T, as specified by `rng`
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
    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::None)
    );
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
    // ANCHOR_END: tetris_player_garbage
    Ok(())
}
