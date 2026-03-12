use core_tetris::{MockRng, TetrisPlayer, TetrominoKind};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[test]
fn deserialization_works() -> Result<(), Box<dyn Error>> {
    let file = File::open("tests/serialization/deserialization.json")?;
    let reader = BufReader::new(file);
    let _player: TetrisPlayer = serde_json::from_reader(reader)?;

    Ok(())
}

#[test]
fn serialization_works() -> Result<(), Box<dyn Error>> {
    let mut rng = MockRng::new(Vec::from(TetrominoKind::ALL));
    let player = TetrisPlayer::default(&mut rng, core_tetris::BagType::NoBag);

    let file = OpenOptions::new()
        .write(true)
        .open("tests/serialization/serialization.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &player)?;

    Ok(())
}
