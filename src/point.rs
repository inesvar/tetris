use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    pub fn translate(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub trait Transformable {
    fn go_down(&mut self);
    fn go_up(&mut self);
    fn go_right(&mut self);
    fn go_left(&mut self);
    fn rotate_clockwise(&mut self, other: &Point);
    fn rotate_counterclockwise(&mut self, other: &Point);
}

impl Transformable for Point {
    fn go_down(&mut self) {
        self.y += 1;
    }

    fn go_up(&mut self) {
        self.y -= 1;
    }

    fn go_left(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    fn go_right(&mut self) {
        self.x += 1;
    }

    fn rotate_clockwise(&mut self, other: &Point) {
        let temp = self.x - other.x;
        self.x = other.x - self.y + other.y;
        self.y = other.y + temp;
    }

    fn rotate_counterclockwise(&mut self, other: &Point) {
        let temp = self.x - other.x;
        self.x = other.x + self.y - other.y;
        self.y = other.y - temp;
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/* impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
} */

/* impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
} */
