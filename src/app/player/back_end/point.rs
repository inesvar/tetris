//! Defines `struct` [Point] and `trait` [TetrisMoves].
use serde::{Deserialize, Serialize};

/// Point on a discrete grid, implementing `trait` [TetrisMoves], serializable.
#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub(super) struct Point {
    /// horizontal coordinate, from left to right
    x: i8,
    /// vertical coordinate, *from top to bottom*
    y: i8,
}

/// All possible tetromino moves in tetris.
pub(super) trait TetrisMoves {
    fn x(&self) -> i8;
    fn y(&self) -> i8;
    /// Move down one cell.
    fn go_down(&mut self);
    /// Move one cell left.
    fn go_left(&mut self);
    /// Move one cell right.
    fn go_right(&mut self);
    /// Turn 90° clockwise around point `origin`.
    fn turn_clockwise_around(&mut self, origin: &Point);
    /// Turn 90° counterclockwise around point `origin`.
    fn turn_counterclockwise_around(&mut self, origin: &Point);
}

impl Point {
    pub(super) fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    fn turned_clockwise(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    fn turned_counterclockwise(&self) -> Self {
        Point::new(-self.y, self.x)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl TetrisMoves for Point {
    fn x(&self) -> i8 {
        self.x
    }

    fn y(&self) -> i8 {
        self.y
    }

    fn go_down(&mut self) {
        self.y += 1;
    }

    fn go_left(&mut self) {
        self.x -= 1;
    }

    fn go_right(&mut self) {
        self.x += 1;
    }

    fn turn_clockwise_around(&mut self, other: &Point) {
        let vector = *self - *other;
        *self = *other + vector.turned_clockwise();
    }

    fn turn_counterclockwise_around(&mut self, other: &Point) {
        let vector = *self - *other;
        *self = *other + vector.turned_counterclockwise();
    }
}
