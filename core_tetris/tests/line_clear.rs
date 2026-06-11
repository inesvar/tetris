use core_tetris::{
    BagType, MockRng, TetrisCommand, TetrisGrid, TetrisPlayer, TetrisResult, TetrominoKind,
};
use std::str::FromStr;

#[test]
fn line_clear_from_setup() -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
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
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "XXXXXXXX \n",
            "---------\n",
        )
    );
    player.try_apply(TetrisCommand::Clockwise, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    assert_eq!(player.score(), 4);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 4);

    Ok(())
}

#[test]
fn t_spin_double_from_setup() -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let mut garbage_rng = MockRng::default();
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
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();

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
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Counterclockwise, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T]);
    let mut garbage_rng = MockRng::default();
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
    let mut player = TetrisPlayer::try_from_matrix(&mut rng, BagType::NoBag, grid).unwrap();

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
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
    while player.try_fall() {}
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Clockwise, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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

#[test]
fn manual_line_clear() -> TetrisResult {
    let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::O, TetrominoKind::I]);
    let mut garbage_rng = MockRng::default();
    let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

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
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Right, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Clockwise, &mut rng, &mut garbage_rng)?;
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
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
    player.try_apply(TetrisCommand::Left, &mut rng, &mut garbage_rng)?;
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
    assert_eq!(player.score(), 0);
    player.try_apply(TetrisCommand::HardDrop, &mut rng, &mut garbage_rng)?;
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
    assert_eq!(player.score(), 1);
    assert_eq!(player.get_lines_completed(), 0);
    assert_eq!(player.score(), 1);

    Ok(())
}
