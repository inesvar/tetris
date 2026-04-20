use super::{CircularBuffer, TetrominoKind};
use rand::TryRng;
use std::convert::Infallible;

/// [MockRng] implements [TryRng] and is designed to control the output of random generation.
/// - for tetromino generation, [MockRng::cycle] and [MockRng::only] are designed to be used with [super::BagType::NoBag]
pub struct MockRng(CircularBuffer<u32>);

impl MockRng {
    pub fn cycle(array: &[TetrominoKind]) -> Self {
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

    pub fn only(kind: TetrominoKind) -> Self {
        Self::cycle(&[kind])
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
    use rand::seq::IndexedRandom;
    use rstest::rstest;
    #[rstest]
    #[case::o(TetrominoKind::O, 0x20000000)]
    #[case::i(TetrominoKind::I, 0x40000000)]
    #[case::t(TetrominoKind::T, 0x60000000)]
    #[case::l(TetrominoKind::L, 0x80000000)]
    #[case::j(TetrominoKind::J, 0xa0000000)]
    #[case::s(TetrominoKind::S, 0xc0000000)]
    #[case::z(TetrominoKind::Z, 0xe0000000)]
    fn mock_rng_next_is_correct_when_there_is_one_element(
        #[case] kind: TetrominoKind,
        #[case] expected: u32,
    ) {
        let mut mock = MockRng::only(kind);
        assert_eq!(TetrominoKind::ALL.choose(&mut mock), Some(&kind));
        assert_eq!(mock.next(), expected);
    }

    #[rstest]
    #[case::tetromino_kind_all(Vec::from(TetrominoKind::ALL))]
    #[case::tetromino_kind_t_l_j(TetrominoKind::ALL[2..5].to_vec())]
    fn mock_rng_next_is_correct(#[case] values: Vec<TetrominoKind>) {
        let mut mock = MockRng::cycle(&values);

        let generated_values = Vec::from_iter(
            std::iter::repeat_with(|| *TetrominoKind::ALL.choose(&mut mock).unwrap())
                .take(values.len()),
        );

        assert_eq!(generated_values, values);
    }
}
