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
