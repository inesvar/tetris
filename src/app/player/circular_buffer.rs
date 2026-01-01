//! Define [CircularBuffer] and methods to use it.
use core::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Push back pop front circular buffer.
#[derive(Serialize, Deserialize, Debug)]
pub struct CircularBuffer<const K: usize, T: Default + Copy + Serialize + Debug>
where
    [T; K]: Serialize + for<'a> Deserialize<'a>,
{
    array: [T; K],
    begin: usize,
    size: usize,
}

impl<const K: usize, T: Default + Copy + Serialize + Debug> Display for CircularBuffer<K, T>
where
    [T; K]: Serialize + for<'a> Deserialize<'a>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "begin {}, size {}, content", self.begin, self.size)?;
        for i in 0..K {
            write!(f, " {:?}", self.array[i])?;
        }
        Ok(())
    }
}

impl<const K: usize, T: Default + Copy + Serialize + Debug> CircularBuffer<K, T>
where
    [T; K]: Serialize + for<'a> Deserialize<'a>,
{
    /// Construct a new circular buffer of size K for type T.
    pub(super) fn new(array: [T; K]) -> Self {
        CircularBuffer::<K, T> {
            array,
            begin: 0,
            size: K,
        }
    }

    pub(super) fn get_front_push_back(&mut self, replacement: &mut T) {
        std::mem::swap(replacement, &mut self.array[self.begin]);
        self.begin += 1;
        self.begin %= K;
    }

    pub(super) fn get_back_push_front(&mut self, replacement: &mut T) {
        self.begin += K - 1;
        self.begin %= K;
        std::mem::swap(replacement, &mut self.array[self.begin]);
    }

    /// Get the i-th element in the buffer.
    pub(super) fn get(&self, i: usize) -> Option<T> {
        //println!("getting {i} from {}", self);
        if i < self.size {
            Some(self.array[(self.begin + i) % K])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CircularBuffer::new([0; 5]))]
    #[case(CircularBuffer::new([10; 4]))]
    fn new_is_correct<const K: usize>(#[case] buffer: CircularBuffer<K, usize>)
    where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        assert_eq!(buffer.size, K);
        assert_eq!(buffer.begin, 0);
    }

    #[rstest]
    #[case(CircularBuffer::new([0, 1, 2, 3, 4]))]
    #[case(CircularBuffer::new([0, 1, 2, 3]))]
    fn get_is_correct<const K: usize>(#[case] buffer: CircularBuffer<K, usize>)
    where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        for i in 0..buffer.size {
            assert_eq!(buffer.get(i), Some(i));
        }
        assert_eq!(buffer.get(buffer.size), None);
    }

    #[rstest]
    #[case(CircularBuffer::new([55, 22, 33]))]
    #[case(CircularBuffer::new([44, 66, 0, 88]))]
    fn get_front_push_back_is_correct<const K: usize>(#[case] mut buffer: CircularBuffer<K, usize>)
    where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        let mut replacement;
        for i in 0..K {
            replacement = i;
            assert_eq!(buffer.size, K, "{}", buffer);
            assert_eq!(buffer.begin, i, "{}", buffer);
            buffer.get_front_push_back(&mut replacement);
            assert_eq!(buffer.get(buffer.size - 1), Some(i), "{}", buffer);
        }
        replacement = K;
        assert_eq!(buffer.size, K, "{}", buffer);
        assert_eq!(buffer.begin, 0, "{}", buffer);
        buffer.get_front_push_back(&mut replacement);
        assert_eq!(buffer.get(buffer.size - 1), Some(K), "{}", buffer);
    }

    #[rstest]
    #[case(CircularBuffer::new([55, 22, 33]))]
    #[case(CircularBuffer::new([44, 66, 0, 88]))]
    fn get_back_push_front_is_correct<const K: usize>(#[case] mut buffer: CircularBuffer<K, usize>)
    where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        let mut replacement;
        for i in 0..K {
            replacement = i;
            assert_eq!(buffer.size, K, "{}", buffer);
            assert_eq!(buffer.begin, (K - i) % K, "{}", buffer);
            buffer.get_back_push_front(&mut replacement);
            assert_eq!(buffer.get(0), Some(i), "{}", buffer);
        }
        replacement = K;
        assert_eq!(buffer.size, K, "{}", buffer);
        assert_eq!(buffer.begin, 0, "{}", buffer);
        buffer.get_back_push_front(&mut replacement);
        assert_eq!(buffer.get(0), Some(K), "{}", buffer);
    }
}
