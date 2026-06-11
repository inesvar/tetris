use core_tetris::{BagType, GameOverError, MockRng, TetrisGrid, TetrisPlayer};
use rstest::rstest;
use std::str::FromStr;

#[rstest]
#[case("", "unable to create `TetrisGrid` from empty string")]
#[case(" \n\n", "expected lines of equal length")]
#[case("---------\n", "expected 3 lines of '-'")]
#[case(concat!(
    "---------\n",
    "         \n",
    "---------\n",
    "         \n",
    "         \n",
    "         \n",
    "         \n",
    "         \n",
    "         \n",
    "---------\n",
), "number of buffer rows should be greater than or equal to `TetrisGrid::MINIMAL_NB_BUFFER_ROWS`")]
#[case(concat!(
    "---------\n",
    "         \n",
    "         \n",
    "---------\n",
    "         \n",
    "         \n",
    "         \n",
    "         \n",
    "         \n",
    "---------\n",
), "number of matrix rows should be greater than or equal to `TetrisGrid::MINIMAL_NB_MATRIX_ROWS`")]
#[case(concat!(
    "---\n", // 
    "   \n", // 
    "   \n", // 
    "---\n", // 
    "   \n", // 
    "   \n", // 
    "   \n", // 
    "   \n", // 
    "   \n", // 
    "   \n", // 
    "---\n", // 
), "number of columns should be greater than or equal to `TetrisGrid::MINIMAL_NB_COLUMNS`")]
#[case(concat!(
    "-----------------------------------------------------------------------------------------------------\n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "-----------------------------------------------------------------------------------------------------\n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "                                                                                                     \n",
    "-----------------------------------------------------------------------------------------------------\n",
), "number of columns, matrix rows and buffer rows should be less than or equal to `TetrisGrid::MAX`")]
fn grid_creation_failure(#[case] string: &str, #[case] expected_msg: &str) {
    let err = TetrisGrid::from_str(string).unwrap_err();
    assert!(
        err.to_string().starts_with(expected_msg),
        "Actual message: {}\nExpected      : {}",
        err,
        expected_msg
    );
}

#[rstest]
#[case(TetrisGrid::from_str(concat!(
    "---------\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    "---------\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    " XXXXXXXX\n",
    "---------\n",
)).unwrap())]
fn tetris_player_creation_failure(#[case] grid: TetrisGrid) {
    assert!(matches!(
        TetrisPlayer::try_from_matrix(&mut MockRng::default(), BagType::NoBag, grid),
        Err(GameOverError::BlockOut)
    ));
}
