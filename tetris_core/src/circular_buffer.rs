//! Define [CircularBuffer] and methods to use it.
use core::fmt::{Debug, Display};
use rand::TryRng;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::Formatter;

/// Push back pop front circular buffer.
#[derive(Serialize, Deserialize, Debug)]
pub struct CircularBuffer<T: Debug> {
    vec: Vec<T>,
    begin: usize,
}

pub type MockRng = CircularBuffer<u32>;

impl<T: Debug> Display for CircularBuffer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "begin {}, content", self.begin)?;
        for elem in &self.vec {
            write!(f, " {elem:?}")?;
        }
        Ok(())
    }
}

impl<T: Debug> CircularBuffer<T> {
    /// Construct a new circular buffer of size K for type T.
    pub(super) fn new(array: Vec<T>) -> Self {
        CircularBuffer::<T> {
            vec: array,
            begin: 0,
        }
    }

    pub(super) fn get_front_push_back(&mut self, replacement: &mut T) {
        std::mem::swap(replacement, &mut self.vec[self.begin]);
        self.begin += 1;
        self.begin %= self.vec.len();
    }

    #[cfg(test)]
    pub(super) fn get_back_push_front(&mut self, replacement: &mut T) {
        self.begin += self.vec.len() - 1;
        self.begin %= self.vec.len();
        std::mem::swap(replacement, &mut self.vec[self.begin]);
    }

    /// Get the i-th element in the buffer.
    pub fn get(&self, i: usize) -> Option<&T> {
        //println!("getting {i} from {}", self);
        if i < self.vec.len() {
            Some(&self.vec[(self.begin + i) % self.vec.len()])
        } else {
            None
        }
    }
}

impl MockRng {
    fn next(&mut self) -> u32 {
        let next = self.vec[self.begin];
        self.begin = (self.begin + 1) % self.vec.len();
        next
    }
}

impl Default for MockRng {
    fn default() -> Self {
        Self {
            vec: vec![0],
            begin: 0,
        }
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
    use rstest::rstest;

    #[rstest]
    #[case(CircularBuffer::new(vec![0; 5]))]
    #[case(CircularBuffer::new(vec![10; 4]))]
    fn new_is_correct(#[case] buffer: CircularBuffer<usize>) {
        assert_eq!(buffer.begin, 0);
    }

    #[rstest]
    #[case(CircularBuffer::new(vec![0, 1, 2, 3, 4]), "begin 0, content 0 1 2 3 4")]
    #[case(CircularBuffer::new(vec![3, 3, 3]), "begin 0, content 3 3 3")]
    fn display_is_correct(#[case] circ_array: CircularBuffer<i32>, #[case] expected: &str) {
        assert_eq!(circ_array.to_string(), expected);
    }

    #[rstest]
    #[case(CircularBuffer::new(vec![0, 1, 2, 3, 4]))]
    #[case(CircularBuffer::new(vec![0, 1, 2, 3]))]
    fn get_is_correct(#[case] buffer: CircularBuffer<usize>) {
        let size = buffer.vec.len();
        for i in 0..size {
            assert_eq!(buffer.get(i), Some(&i));
        }
        assert_eq!(buffer.get(size), None);
    }

    #[rstest]
    #[case(CircularBuffer::new(vec![55, 22, 33]))]
    #[case(CircularBuffer::new(vec![44, 66, 0, 88]))]
    fn get_front_push_back_is_correct(#[case] mut buffer: CircularBuffer<usize>) {
        let size = buffer.vec.len();
        let mut replacement;
        for i in 0..size {
            replacement = i;
            assert_eq!(buffer.begin, i, "{}", buffer);
            buffer.get_front_push_back(&mut replacement);
            assert_eq!(buffer.get(size - 1), Some(&i), "{}", buffer);
        }
        replacement = size;
        assert_eq!(buffer.begin, 0, "{}", buffer);
        buffer.get_front_push_back(&mut replacement);
        assert_eq!(buffer.get(size - 1), Some(&size), "{}", buffer);
    }

    #[rstest]
    #[case(CircularBuffer::new(vec![55, 22, 33]))]
    #[case(CircularBuffer::new(vec![44, 66, 0, 88]))]
    fn get_back_push_front_is_correct(#[case] mut buffer: CircularBuffer<usize>) {
        let size = buffer.vec.len();
        let mut replacement;
        for i in 0..size {
            replacement = i;
            assert_eq!(buffer.begin, (size - i) % size, "{}", buffer);
            buffer.get_back_push_front(&mut replacement);
            assert_eq!(buffer.get(0), Some(&i), "{}", buffer);
        }
        replacement = size;
        assert_eq!(buffer.begin, 0, "{}", buffer);
        buffer.get_back_push_front(&mut replacement);
        assert_eq!(buffer.get(0), Some(&size), "{}", buffer);
    }

    #[test]
    fn default_mock_rng_next_is_correct() {
        let mut mock = MockRng::default();
        assert_eq!(mock.next(), 0);
        assert_eq!(mock.next(), 0);
    }
}
