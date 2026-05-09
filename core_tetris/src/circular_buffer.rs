//! Define [CircularBuffer] and methods to use it.
use super::CoreTetrisError;
use core::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Push back pop front circular buffer.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircularBuffer<T: Debug> {
    vec: Vec<T>,
    begin: usize,
}

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
    /// Creates a new circular buffer containing `array` values.
    ///
    /// # Panics
    ///
    /// If `array` is empty.
    pub(super) fn new(array: Vec<T>) -> Self {
        Self::try_new(array).expect("CircularBuffer should not be empty")
    }

    /// Tries to create a new circular buffer of size K for type T.
    /// Fails if `array` is empty.
    pub(super) fn try_new(array: Vec<T>) -> Result<Self, CoreTetrisError> {
        if array.is_empty() {
            return Err(CoreTetrisError::EmptyCircularBuffer);
        }
        Ok(CircularBuffer::<T> {
            vec: array,
            begin: 0,
        })
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

    pub(super) fn peek(&self) -> &T {
        &self.vec[self.begin]
    }

    /// Get the `i`-th element in the buffer.
    pub fn get(&self, i: usize) -> Option<&T> {
        //println!("getting {i} from {}", self);
        if i < self.vec.len() {
            Some(&self.vec[(self.begin + i) % self.vec.len()])
        } else {
            None
        }
    }

    /// Get the buffer size.
    pub fn size(&self) -> usize {
        self.vec.len()
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
}
