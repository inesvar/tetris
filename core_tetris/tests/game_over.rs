use core_tetris::{
    BagType, GameOverError, MockRng, TetrisGrid, TetrisPlayer, TetrisResult, TetrominoMove,
};
use std::str::FromStr;

#[test]
fn lock_out() -> TetrisResult {
    let rng = &mut MockRng::default();
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        "---------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert_eq!(
        player.try_apply(TetrominoMove::HardDrop.into(), rng, garbage_rng),
        Err(GameOverError::LockOut)
    );

    assert_eq!(
        player.grid().to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    Ok(())
}

#[test]
fn block_out() -> TetrisResult {
    let rng = &mut MockRng::default();
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        " XXX  XXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        "---------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXX  XXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert_eq!(
        player.try_apply(TetrominoMove::HardDrop.into(), rng, garbage_rng),
        Err(GameOverError::BlockOut)
    );

    assert_eq!(
        player.grid().to_string(),
        concat!(
            "---------\n",
            "         \n",
            "    OO   \n",
            "---------\n",
            " XXXOOXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXXOOXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    Ok(())
}

#[test]
fn top_out() -> TetrisResult {
    let rng = &mut MockRng::default();
    let garbage_rng = &mut MockRng::default();
    let grid = TetrisGrid::from_str(concat!(
        "---------\n",
        "         \n",
        "         \n",
        "---------\n",
        " XXX  XXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        " XXXXXXXX\n",
        "---------\n",
    ))
    .unwrap();
    let mut player = TetrisPlayer::try_from_matrix(rng, BagType::NoBag, grid).unwrap();

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            " XXX  XXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    player.push_garbage(2);
    // player.try_apply(TetrominoMove::HardDrop.into(), rng, garbage_rng)?;
    assert_eq!(
        player.try_apply(TetrominoMove::HardDrop.into(), rng, garbage_rng),
        Err(GameOverError::TopOut)
    );

    assert_eq!(
        player.grid().to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            " XXXOOXXX\n",
            "---------\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    assert_eq!(
        player.to_string(),
        concat!(
            "---------\n",
            "    OO   \n",
            " XXXOOXXX\n",
            "---------\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            " XXXXXXXX\n",
            "---------\n",
        )
    );

    Ok(())
}
