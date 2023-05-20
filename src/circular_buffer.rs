use serde::Serialize;
use core::fmt::Display;
use std::fmt::Formatter;

/* push back pop front circular buffer */
#[derive(Serialize, Debug)]
pub struct CircularBuffer<const K: usize, T : Default + Copy + Serialize + Display> where [T; K] : Serialize {
    array: [T; K],
    begin: usize,
    size: usize,
}

impl<const K: usize, T: Default + Copy + Serialize + Display> Display for CircularBuffer<K, T> where [T; K] : Serialize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "begin {}, size {}, content {}{}{}{}{}", self.begin, self.size, self.array[0], self.array[1], self.array[2], self.array[3], self.array[4])
    }
}

impl<const K: usize, T: Default + Copy + Serialize + Display> CircularBuffer<K, T> where [T; K] : Serialize {
    pub fn new() -> Self {
        CircularBuffer::<K, T> {
            array: [T::default(); K],
            begin: 0,
            size: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<T> {
        //println!("getting {i} from {}", self);
        if i < self.size {
            Some(self.array[(self.begin + i)%K])
        } else {
            None
        }
    }

    pub fn push(&mut self, t: T) {
        if self.size != K {
            self.array[(self.begin + self.size)%K] = t;
            self.size += 1;
        }
        //println!("pushed {t}, now {}", self);
    }

    pub fn pop(&mut self) -> Option<T> {
        //println!("popping from {}", self);
        if self.size != 0 {
            let pop = self.array[self.begin];
            self.begin += 1;
            self.begin = self.begin%K;
            self.size -= 1;
            Some(pop)
        } else {
            None
        }
    }
}

