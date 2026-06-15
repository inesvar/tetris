use core_tetris::{
    BagType, LineClear, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult,
    TetrominoKind,
};
use std::str::FromStr;

#[test]
fn almost_t_spin() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "----------\n",
        "          \n",
        "          \n",
        "----------\n",
        "          \n",
        "          \n",
        "          \n",
        "          \n",
        "          \n",
        " XXXXXXXXX\n",
        "----------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            " XXXXXXXXX\n",
            "----------\n",
        )
    );

    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;
    while player.try_left() {}
    while player.try_fall() {}
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "T         \n",
            "TT        \n",
            "TXXXXXXXXX\n",
            "----------\n",
        )
    );

    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::Single)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "T         \n",
            "TT        \n",
            "----------\n",
        )
    );

    Ok(())
}

#[test]
fn t_spin_single() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "----------\n",
        "          \n",
        "          \n",
        "----------\n",
        "          \n",
        "          \n",
        "          \n",
        "          \n",
        "          \n",
        " XXXXXXXXX\n",
        "----------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            " XXXXXXXXX\n",
            "----------\n",
        )
    );

    while player.try_left() {}
    while player.try_fall() {}
    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "T         \n",
            "TT        \n",
            "TXXXXXXXXX\n",
            "----------\n",
        )
    );

    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::TSpinSingle)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "T         \n",
            "TT        \n",
            "----------\n",
        )
    );

    Ok(())
}

#[test]
fn t_spin_double_from_setup() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "----------\n",
        "          \n",
        "          \n",
        "----------\n",
        "          \n",
        "      XX  \n",
        "  X    XX \n",
        "X X XX XX \n",
        "XXXXX  XXX\n",
        "XXXXXX XXX\n",
        "----------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "      XX  \n",
            "  X    XX \n",
            "X X XX XX \n",
            "XXXXX  XXX\n",
            "XXXXXX XXX\n",
            "----------\n",
        )
    );
    while player.try_fall() {}
    player.try_apply(TetrisCommand::Right, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "     TXX  \n",
            "  X TTTXX \n",
            "X X XX XX \n",
            "XXXXX  XXX\n",
            "XXXXXX XXX\n",
            "----------\n",
        )
    );
    player.try_apply(TetrisCommand::Counterclockwise, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "      XX  \n",
            "  X    XX \n",
            "X X XXTXX \n",
            "XXXXXTTXXX\n",
            "XXXXXXTXXX\n",
            "----------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 0);
    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::TSpinDouble)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "      XX  \n",
            "  X    XX \n",
            "X X XXTXX \n",
            "----------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 2);
    assert_eq!(player.score(), 2);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 2);

    Ok(())
}

#[test]
fn t_spin_triple_from_setup() -> TetrisResult {
    let rng = &mut MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "----------\n",
        "          \n",
        "          \n",
        "----------\n",
        "          \n",
        "  XX      \n",
        " XX    X  \n",
        "XXX XXXXXX\n",
        "XXX  XXXXX\n",
        "XXX XXXXXX\n",
        "----------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "  XX      \n",
            " XX    X  \n",
            "XXX XXXXXX\n",
            "XXX  XXXXX\n",
            "XXX XXXXXX\n",
            "----------\n",
        )
    );
    player.try_apply(TetrisCommand::Right, rng, garbage_rng)?;
    while player.try_fall() {}
    player.try_apply(TetrisCommand::Left, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "  XXT     \n",
            " XXTTT X  \n",
            "XXX XXXXXX\n",
            "XXX  XXXXX\n",
            "XXX XXXXXX\n",
            "----------\n",
        )
    );
    player.try_apply(TetrisCommand::Clockwise, rng, garbage_rng)?;
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "          \n",
            "          \n",
            "----------\n",
            "          \n",
            "  XX      \n",
            " XX    X  \n",
            "XXXTXXXXXX\n",
            "XXXTTXXXXX\n",
            "XXXTXXXXXX\n",
            "----------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 0);
    assert_eq!(
        player.try_apply(TetrisCommand::HardDrop, rng, garbage_rng),
        Ok(LineClear::TSpinTriple)
    );
    assert_eq!(
        player.to_string(),
        concat!(
            "----------\n",
            "    T     \n",
            "   TTT    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "  XX      \n",
            " XX    X  \n",
            "----------\n",
        )
    );
    assert_eq!(player.get_lines_completed(), 3);
    assert_eq!(player.score(), 3);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 3);

    Ok(())
}
