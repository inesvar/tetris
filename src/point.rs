
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
    fn down(&mut self);
    fn right(&mut self);
    fn left(&mut self);
    fn turn_clockwise(&mut self, other: &Point);
    fn turn_counterclockwise(&mut self, other: &Point);
}

impl Transformable for Point {
    /* methode to new
    
    fn down(self) -> Self {
        self + Point::new(0, -1)
    }*/

    fn down(&mut self) {
        self.y += 1;
    }

    fn left(&mut self) {
        self.x += -1;
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
