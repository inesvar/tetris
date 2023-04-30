pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point::<T> { x, y }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Self::Output {
        Point::<T>::new(self.x + other.x, self.y + other.y)
    }
}
pub trait Transformable {
    fn down(self);
    fn right(self);
    fn left(self);
    fn turn_clockwise(self);
    fn turn_counterclockwise(self);
}

impl Transformable for Point<i8> {
    fn down(mut self) {
        self.y += -1;
    }

    /* methode sans emplace
    
    fn right(self) -> Self {
        self + Point::new(1, 0)
    }*/

    fn left(mut self) {
        self.x += -1;
    }

    fn right(mut self) {
        self.x += 1;
    }

    fn turn_clockwise(mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = -temp;
    }

    fn turn_counterclockwise(mut self) {
        let temp = self.x;
        self.x = -self.y;
        self.y = temp;
    }
}
