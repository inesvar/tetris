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

    pub(super) fn swap_front(&mut self, replacement: &mut T) {
        std::mem::swap(replacement, &mut self.array[self.begin]);
        self.begin += 1;
        self.begin %= K;
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

    /// Push an element to the back of the buffer.
    pub(super) fn push(&mut self, t: T) {
        if self.size != K {
            self.array[(self.begin + self.size) % K] = t;
            self.size += 1;
        }
        //println!("pushed {t}, now {}", self);
    }

    /// Push an element to the front of the buffer.
    pub(super) fn push_front(&mut self, t: T) {
        if self.size != K {
            let begin: usize = if self.begin > 0 {
                self.begin - 1
            } else {
                K - 1
            };
            self.array[begin] = t;
            self.begin = begin;
            self.size += 1;
        }
        //println!("pushed {t}, now {}", self);
    }

    /// Pop an element from the front of the buffer.
    pub(super) fn pop_front(&mut self) -> Option<T> {
        //println!("popping from {}", self);
        if self.size != 0 {
            let pop = self.array[self.begin];
            self.begin += 1;
            self.begin %= K;
            self.size -= 1;
            Some(pop)
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
    #[case(CircularBuffer::new([55, 22, 33]), 55)]
    #[case(CircularBuffer::new([44, 66, 0, 88]), 44)]
    fn pop_front_after_new_is_correct<const K: usize>(
        #[case] mut buffer: CircularBuffer<K, usize>,
        #[case] first: usize,
    ) where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        assert_eq!(buffer.pop_front(), Some(first));
        assert_eq!(buffer.size, K - 1);
        assert_eq!(buffer.begin, 1);
    }

    #[rstest]
    #[case(CircularBuffer::new([55, 22, 33]))]
    #[case(CircularBuffer::new([44, 66, 0, 88]))]
    fn pop_front_returns_some_when_not_empty<const K: usize>(
        #[case] mut buffer: CircularBuffer<K, usize>,
    ) where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        for i in 0..K {
            let expected = buffer.get(0);
            assert!(expected.is_some(), "{}", buffer);
            assert_eq!(buffer.size, K - i, "{}", buffer);
            assert_eq!(buffer.begin, i, "{}", buffer);
            assert_eq!(buffer.pop_front(), expected, "{}", buffer);
        }
        let expected = buffer.get(0);
        assert!(expected.is_none(), "{}", buffer);
        assert_eq!(buffer.size, 0, "{}", buffer);
        assert_eq!(buffer.begin, 0, "{}", buffer);
        assert_eq!(buffer.pop_front(), expected, "{}", buffer);
    }

    #[rstest]
    #[case(CircularBuffer::new([55, 22, 33]))]
    #[case(CircularBuffer::new([44, 66, 0, 88]))]
    fn swap_front_is_correct<const K: usize>(#[case] mut buffer: CircularBuffer<K, usize>)
    where
        [usize; K]: Serialize + for<'a> Deserialize<'a>,
    {
        let mut replacement;
        for i in 0..K {
            replacement = i;
            assert_eq!(buffer.size, K, "{}", buffer);
            assert_eq!(buffer.begin, i, "{}", buffer);
            buffer.swap_front(&mut replacement);
            assert_eq!(buffer.get(buffer.size - 1), Some(i), "{}", buffer);
        }
        replacement = K;
        assert_eq!(buffer.size, K, "{}", buffer);
        assert_eq!(buffer.begin, 0, "{}", buffer);
        buffer.swap_front(&mut replacement);
        assert_eq!(buffer.get(buffer.size - 1), Some(K), "{}", buffer);
    }
}
