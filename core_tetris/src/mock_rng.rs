use super::{CircularBuffer, TetrominoKind};
use rand::TryRng;
use std::{convert::Infallible, ops::Range};

/// [MockRng] implements [TryRng] and is designed to control the output of random generation.
///
/// [MockRng::tetromino_cycle] and [MockRng::o_tetrominos] controls
/// tetromino generation (this only works with [super::BagType::NoBag]).
///
/// [MockRng::garbage_cycle] and [MockRng::right_aligned_garbage] controls garbage generation
/// (more precisely, the gap in the garbage lines).
pub struct MockRng(CircularBuffer<u32>);

#[allow(missing_docs)] // TODO fix this ! add examples !!
impl MockRng {
    pub fn tetromino_cycle(array: &[TetrominoKind]) -> Self {
        let mut indices = Vec::new();
        for kind in array {
            // Little retro-engineering of `rand::seq::IndexedRandom::choose` on `TetrominoKind::ALL`.
            let index = TetrominoKind::ALL
                .iter()
                .position(|k| k == kind)
                .expect("All TetrominoKind variants should be in TetrominoKind::ALL")
                as u32;
            indices.push((index + 1) << 29);
        }
        Self(CircularBuffer::new(indices))
    }

    pub fn o_tetrominos() -> Self {
        Self::tetromino_cycle(&[TetrominoKind::O])
    }

    pub fn garbage_cycle(values: &[u32], range: Range<u32>) -> Self {
        let mut spaced_values = Vec::new();
        let len = range
            .end
            .saturating_sub(1)
            .saturating_sub(range.start)
            .max(1);
        for value in values {
            if !range.contains(value) {
                panic!("`values` should be contained in `range`")
            }
            let spaced_value = u32::MAX / len * *value;
            spaced_values.push(spaced_value);
        }
        Self(CircularBuffer::new(spaced_values))
    }
    pub fn right_aligned_garbage() -> Self {
        let values = vec![0];
        Self(CircularBuffer::new(values))
    }

    fn next(&mut self) -> u32 {
        let mut next = *self.0.peek();
        self.0.get_front_push_back(&mut next);

        next
    }
}

impl TryRng for MockRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let high = self.next() as u64;
        let low = self.next() as u64;
        Ok(high << 32 | low)
    }

    // This is a naive implementation : it's only for tests.
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        for i in dst {
            *i = self.next() as u8;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{seq::IndexedRandom, RngExt};
    use rstest::rstest;

    #[rstest]
    #[case::o(TetrominoKind::O, 0x20000000)]
    #[case::i(TetrominoKind::I, 0x40000000)]
    #[case::t(TetrominoKind::T, 0x60000000)]
    #[case::l(TetrominoKind::L, 0x80000000)]
    #[case::j(TetrominoKind::J, 0xa0000000)]
    #[case::s(TetrominoKind::S, 0xc0000000)]
    #[case::z(TetrominoKind::Z, 0xe0000000)]
    fn choose_tetromino_using_one_tetromino_is_correct(
        #[case] kind: TetrominoKind,
        #[case] expected: u32,
    ) {
        let mut mock = MockRng::tetromino_cycle(&[kind]);
        assert_eq!(TetrominoKind::ALL.choose(&mut mock), Some(&kind));
        assert_eq!(mock.next(), expected);
    }

    #[rstest]
    #[case::tetromino_kind_all(Vec::from(TetrominoKind::ALL))]
    #[case::tetromino_kind_t_l_j(TetrominoKind::ALL[2..5].to_vec())]
    fn choose_tetromino_using_tetromino_cycle_is_correct(#[case] values: Vec<TetrominoKind>) {
        let mut mock = MockRng::tetromino_cycle(&values);

        let generated_values = Vec::from_iter(
            std::iter::repeat_with(|| *TetrominoKind::ALL.choose(&mut mock).unwrap())
                .take(values.len()),
        );

        assert_eq!(generated_values, values);
    }

    #[rstest]
    #[case(vec![0], 0..10)]
    #[case(vec![9], 0..10)]
    #[case(vec![10], 0..11)]
    #[case(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 0..10)]
    #[case(Vec::from([0, 1, 1, 1, 1, 1, 1, 2, 3, 3]), 0..4)]
    fn random_garbage_using_garbage_cycle_is_correct(
        #[case] garbage_gaps: Vec<u32>,
        #[case] range: Range<u32>,
    ) {
        let mut mock = MockRng::garbage_cycle(&garbage_gaps, range.clone());

        let generated_gaps = Vec::from_iter(
            std::iter::repeat_with(|| mock.random_range(range.clone())).take(garbage_gaps.len()),
        );

        assert_eq!(generated_gaps, garbage_gaps);
    }
}
