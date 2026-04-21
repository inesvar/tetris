//! Define `struct` [TetrominoGenerator] and `enum` [BagType].
#![doc = simple_mermaid::mermaid!("tetromino_generator.mmd")]

use super::{Tetromino, TetrominoKind};
use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Tetromino generator.
#[derive(Serialize, Deserialize, Default)]
pub struct TetrominoGenerator {
    tetrominos: VecDeque<TetrominoKind>,
    bag_type: BagType,
}

/// Type of tetromino bag.
///
/// Using a shuffled bag instead of choosing each tetromino randomly
/// helps prevent tetromino repetition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum BagType {
    /// Each tetromino is chosen randomly using the provided random generator.
    NoBag,
    #[default]
    /// Default.
    ///
    /// One occurence of each tetromino per bag, shuffled using the provided random generator.
    Bag7,
    /// Two occurences of each tetromino per bag, shuffled using the provided random generator.
    Bag14,
}

impl TetrominoGenerator {
    /// Creates a new [TetrominoGenerator] with [BagType] `bag_type`.
    pub(crate) fn new(bag_type: BagType) -> Self {
        Self {
            tetrominos: VecDeque::new(),
            bag_type,
        }
    }

    pub(crate) fn bag_type(&self) -> BagType {
        self.bag_type
    }

    /// Returns an array of `N` [Tetromino]'s.
    pub(crate) fn get_chunk<R: Rng>(&mut self, rng: &mut R, size: usize) -> Vec<Tetromino> {
        (0..size).map(|_| self.get(rng)).collect()
    }

    /// Returns one [Tetromino].
    pub(crate) fn get<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        if self.tetrominos.is_empty() {
            self.draw_new_bag(rng);
        }

        Tetromino::new(
            self.tetrominos
                .pop_front()
                .expect("`draw_new_bag` shouldn't have left `tetrominos` empty"),
        )
    }

    fn draw_new_bag<R: Rng>(&mut self, rng: &mut R) {
        debug_assert!(self.tetrominos.is_empty());
        if let Some(nb_occurrences) = self.bag_type.occurrences_per_kind() {
            self.tetrominos = TetrominoKind::ALL.repeat(nb_occurrences).into();
            self.tetrominos.make_contiguous().shuffle(rng);
        } else {
            let mut random_tetromino = || {
                *TetrominoKind::ALL
                    .choose(rng)
                    .expect("`ALL_TETROMINO_KINDS` shouldn't be empty")
            };
            // taking 7 tetrominos to avoid calling `draw_new_bag` too often
            for _ in 0..7 {
                self.tetrominos.push_back(random_tetromino());
            }
        }
    }
}

impl BagType {
    fn occurrences_per_kind(&self) -> Option<usize> {
        match self {
            BagType::NoBag => None,
            BagType::Bag7 => Some(1),
            BagType::Bag14 => Some(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockRng;
    use rstest::rstest;

    #[rstest]
    #[case::completely_random(BagType::NoBag, None)]
    #[case::bag7(BagType::Bag7, Some(1))]
    #[case::bag14(BagType::Bag14, Some(2))]
    fn occurrences_per_kind_is_correct(
        #[case] bag_type: BagType,
        #[case] occurences: Option<usize>,
    ) {
        assert_eq!(bag_type.occurrences_per_kind(), occurences);
    }

    #[rstest]
    #[case::completely_random(BagType::NoBag, 7)]
    #[case::bag7(BagType::Bag7, 7)]
    #[case::bag14(BagType::Bag14, 14)]
    fn tetrominos_is_not_empty_after_draw_new_bag(
        #[case] bag_type: BagType,
        #[case] bag_size: usize,
    ) {
        let mut bag = TetrominoGenerator::new(bag_type);

        bag.draw_new_bag(&mut MockRng::o_tetrominos());

        assert!(!bag.tetrominos.is_empty());
        assert_eq!(bag.tetrominos.len(), bag_size);
    }

    #[rstest]
    #[case::completely_random(BagType::NoBag)]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_doesnt_panic_after_new(#[case] bag_type: BagType) {
        let mut bag = TetrominoGenerator::new(bag_type);

        bag.get(&mut MockRng::o_tetrominos());
    }

    #[rstest]
    #[case::completely_random(BagType::NoBag)]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_chunk_doesnt_panic_when_chunk_is_very_big(#[case] bag_type: BagType) {
        let mut bag = TetrominoGenerator::new(bag_type);

        bag.get_chunk(&mut MockRng::o_tetrominos(), 50);
    }

    #[rstest]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_chunk_14_returns_2_occurences_of_each_type(#[case] bag_type: BagType) {
        let mut bag = TetrominoGenerator::new(bag_type);

        let tetrominos = bag.get_chunk(&mut MockRng::o_tetrominos(), 14);

        for kind in TetrominoKind::ALL {
            assert_eq!(tetrominos.iter().filter(|&t| t.kind() == kind).count(), 2);
        }
    }

    #[rstest]
    #[case::bag7(BagType::Bag7)]
    fn get_chunk_7_with_bag_type_7_is_correct(#[case] bag_type: BagType) {
        let mut bag = TetrominoGenerator::new(bag_type);

        let tetrominos = bag.get_chunk(&mut MockRng::o_tetrominos(), 7);

        for kind in TetrominoKind::ALL {
            assert_eq!(tetrominos.iter().filter(|&t| t.kind() == kind).count(), 1);
        }
    }

    #[rstest]
    #[case::tetromino_kind_o(&mut MockRng::o_tetrominos(), vec![TetrominoKind::O; 7])]
    #[case::tetromino_kind_t(&mut MockRng::tetromino_cycle(&[TetrominoKind::T]), vec![TetrominoKind::T; 7])]
    #[case::tetromino_kind_all(&mut MockRng::tetromino_cycle(&TetrominoKind::ALL), Vec::from(TetrominoKind::ALL))]
    fn get_chunk_using_no_bag_and_mock_rng_is_correct(
        #[case] rng: &mut MockRng,
        #[case] expected: Vec<TetrominoKind>,
    ) {
        let mut bag = TetrominoGenerator::new(BagType::NoBag);

        let generated: Vec<_> = bag
            .get_chunk(rng, expected.len())
            .into_iter()
            .map(|t| t.kind())
            .collect();

        assert_eq!(generated, expected);
    }
}
