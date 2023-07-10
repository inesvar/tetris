use rand::seq::SliceRandom;
use rand_pcg::Pcg32;

use crate::tetromino_kind::TetrominoKind;

pub fn new_random_bag(mut size_of_bag: u32, rng: &mut Pcg32) -> Vec<TetrominoKind> {
    if size_of_bag == 0 {
        size_of_bag = 1;
    }
    let mut tetromino_bag = vec![];
    let mut list = vec![];
    for _ in 0..=(size_of_bag / 7) {
        for i in 0..=6 {
            list.push(i); // the list has k elements, where k is the lower multiple of 7 higher than size_of_bag
        }
    }
    list.shuffle(rng);

    for i in 0..size_of_bag {
        // only the first size_of_bag elements are used
        match list[i as usize] {
            0 => tetromino_bag.push(TetrominoKind::I),
            1 => tetromino_bag.push(TetrominoKind::O),
            2 => tetromino_bag.push(TetrominoKind::T),
            3 => tetromino_bag.push(TetrominoKind::S),
            4 => tetromino_bag.push(TetrominoKind::Z),
            5 => tetromino_bag.push(TetrominoKind::J),
            _ => tetromino_bag.push(TetrominoKind::L),
        }
    }
    tetromino_bag
}
