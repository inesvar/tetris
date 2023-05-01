#[derive(Clone, Copy)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point{
    pub fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    pub fn set(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
pub trait Transformable {
    fn go_down(&mut self);
    fn go_right(&mut self);
    fn go_left(&mut self);
    fn rotate_clockwise(&mut self, other: &Point);
    fn rotate_counterclockwise(&mut self, other: &Point);
}

pub trait Collision {
    fn fall(&mut self) -> bool;
    fn right(&mut self) -> bool;
    fn left(&mut self) -> bool;
    fn turn_clockwise(&mut self, other: &Point) -> bool;
    fn turn_counterclockwise(&mut self, other: &Point) -> bool;
}

impl Transformable for Point {
    /* methode to new
    
    fn go_down(self) -> Self {
        self + Point::new(0, -1)
    }*/

    fn go_down(&mut self) {
        self.y += 1;
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

impl Collision for Point {

    fn fall(&mut self) {
        self.y += 1;
    }

    fn left(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn turn_clockwise(&mut self, other: &Point) {
        let temp = self.x - other.x;
        self.x = other.x - self.y + other.y;
        self.y = other.y + temp;
    }

    fn turn_counterclockwise(&mut self, other: &Point) {
        let temp = self.x - other.x;
        self.x = other.x + self.y - other.y;
        self.y = other.y - temp;
    }
}
