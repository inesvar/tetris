use serde::Serialize;

#[derive(Serialize)]
pub struct CircularBuffer<const K: usize, T : Default + Copy + Serialize> where [T; K] : Serialize {
    array: [T; K],
    begin: usize,
    size: usize,
}

impl<const K: usize, T: Default + Copy + Serialize> CircularBuffer<K, T> where [T; K] : Serialize {
    pub fn new() -> Self {
        CircularBuffer::<K, T> {
            array: [T::default(); K],
            begin: 0,
            size: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<T> {
        if i < self.size {
            Some(self.array[(self.begin + i)%5])
        } else {
            None
        }
    }

    pub fn push(&mut self, t: T) {
        if self.size != K {
            self.array[(self.begin + self.size)%5] = t;
            self.size += 1;
        }
        println!("begin {}, size {}", self.begin, self.size);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size != 0 {
            let pop = self.array[self.begin];
            self.begin += 1;
            self.begin = self.begin%5;
            Some(pop)
        } else {
            None
        }
    }
}

