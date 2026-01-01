#![allow(unused)]
use super::{Deserialize, Serialize, TetrominoKind};
use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg32;
use std::array;

#[derive(Serialize, Deserialize, Default)]
pub(in crate::app::player) struct TetrominoBag {
    tetrominos: Vec<TetrominoKind>,
    bag_type: BagType,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
pub(in crate::app::player) enum BagType {
    CompletelyRandom,
    #[default]
    /// Default.
    Bag7,
    Bag14,
}

impl TetrominoBag {
    pub(in crate::app::player) fn new(bag_type: BagType) -> Self {
        Self {
            tetrominos: Vec::new(),
            bag_type,
        }
    }

    pub(in crate::app::player) fn get_chunk<const N: usize>(
        &mut self,
        rng: &mut Pcg32,
    ) -> [TetrominoKind; N] {
        array::from_fn(|_| self.get(rng))
    }

    pub(in crate::app::player) fn get(&mut self, rng: &mut Pcg32) -> TetrominoKind {
        self.tetrominos.pop().unwrap_or_else(|| {
            self.draw_new_bag(rng);
            self.tetrominos
                .pop()
                .expect("`draw_new_bag` shouldn't have left `tetrominos` empty")
        })
    }

    fn draw_new_bag(&mut self, rng: &mut Pcg32) {
        if let Some(nb_occurrences) = self.bag_type.occurrences_per_kind() {
            self.tetrominos = TetrominoKind::ALL.repeat(nb_occurrences);
            self.tetrominos.shuffle(rng);
        } else {
            let random_tetromino = || {
                *TetrominoKind::ALL
                    .choose(rng)
                    .expect("`ALL_TETROMINO_KINDS` shouldn't be empty")
            };
            // taking 7 tetrominos to avoid calling `draw_new_bag` too often
            self.tetrominos = Vec::from_iter(std::iter::repeat_with(random_tetromino).take(7));
        }
    }
}

impl BagType {
    fn occurrences_per_kind(&self) -> Option<usize> {
        match self {
            BagType::CompletelyRandom => None,
            BagType::Bag7 => Some(1),
            BagType::Bag14 => Some(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::completely_random(BagType::CompletelyRandom, None)]
    #[case::bag7(BagType::Bag7, Some(1))]
    #[case::bag14(BagType::Bag14, Some(2))]
    fn occurrences_per_kind_is_correct(
        #[case] bag_type: BagType,
        #[case] occurences: Option<usize>,
    ) {
        assert_eq!(bag_type.occurrences_per_kind(), occurences);
    }

    #[rstest]
    #[case::completely_random(BagType::CompletelyRandom, 7)]
    #[case::bag7(BagType::Bag7, 7)]
    #[case::bag14(BagType::Bag14, 14)]
    fn tetrominos_is_not_empty_after_draw_new_bag(
        #[case] bag_type: BagType,
        #[case] bag_size: usize,
    ) {
        let mut bag = TetrominoBag::new(bag_type);

        bag.draw_new_bag(&mut Pcg32::seed_from_u64(0));

        assert!(!bag.tetrominos.is_empty());
        assert_eq!(bag.tetrominos.len(), bag_size);
    }

    #[rstest]
    #[case::completely_random(BagType::CompletelyRandom)]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_doesnt_panic_after_new(#[case] bag_type: BagType) {
        let mut bag = TetrominoBag::new(bag_type);

        bag.get(&mut Pcg32::seed_from_u64(0));
    }

    #[rstest]
    #[case::completely_random(BagType::CompletelyRandom)]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_chunk_doesnt_panic_when_chunk_is_very_big(#[case] bag_type: BagType) {
        let mut bag = TetrominoBag::new(bag_type);

        bag.get_chunk::<50>(&mut Pcg32::seed_from_u64(0));
    }

    #[rstest]
    #[case::bag7(BagType::Bag7)]
    #[case::bag14(BagType::Bag14)]
    fn get_chunk_14_with_classic_bag_types_is_correct(#[case] bag_type: BagType) {
        let mut bag = TetrominoBag::new(bag_type);

        let tetrominos = Vec::from(bag.get_chunk::<14>(&mut Pcg32::seed_from_u64(0)));

        for kind in TetrominoKind::ALL {
            assert_eq!(tetrominos.iter().filter(|&k| *k == kind).count(), 2);
        }
    }

    #[rstest]
    #[case::bag7(BagType::Bag7)]
    fn get_chunk_7_with_bag_type_7_is_correct(#[case] bag_type: BagType) {
        let mut bag = TetrominoBag::new(bag_type);

        let tetrominos = Vec::from(bag.get_chunk::<7>(&mut Pcg32::seed_from_u64(0)));

        for kind in TetrominoKind::ALL {
            assert_eq!(tetrominos.iter().filter(|&k| *k == kind).count(), 1);
        }
    }
}
