//! Defines the basics to move on a grid
use serde::{Deserialize, Serialize};

/// Point on a finite wrap-around 2D grid.
///
/// Notably implements **Transform** trait and std::ops::Add.
///
/// A point by itself is abstract and doesn't implement collisions
/// (it moves without any knownledge of its surroundings).
#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Point {
    /// horizontal coordinate, from left to right
    pub x: i8,
    /// vertical coordinate, *from top to bottom*
    pub y: i8,
}

/// Unhindered moves on a grid (used when collisions aren't necessary)
///
/// ## Uses
/// - Tetromino *center* (go_down, go_right, go_left)
/// - Tetromino *blocks* (rotate_clockwise, rotate_counterclockwise) when checking if a rotation is possible
/// - TetrisGrid *blocks* (go_down, go_up) when receiving/completing lines
///
/// ## Functions
/// - **go_down**(), **go_up**(), **go_right**(), **go_left**()
///
/// move one grid cell in the given direction
///
/// - **rotate_clockwise**(origin: Point), **rotate_counterclockwise**(origin: Point)
///
/// rotate 90° around the origin
pub trait Transform {
    /// Move down one cell without checking if it's empty
    fn go_down(&mut self);
    /// Move up one cell without checking if it's empty
    fn go_up(&mut self);
    /// Move one cell left without checking if it's empty
    fn go_left(&mut self);
    /// Move one cell right without checking if it's empty
    fn go_right(&mut self);
    /// Rotate 90° clockwise around the given origin without checking if the destination is empty
    fn rotate_clockwise(&mut self, other: &Point);
    /// Rotate 90° counterclockwise around the given origin without checking if the destination is empty
    fn rotate_counterclockwise(&mut self, other: &Point);
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    pub fn translate_by(&mut self, other: &Point) {
        *self = *self + *other;
    }
}
impl Transform for Point {
    fn go_down(&mut self) {
        self.y += 1;
    }

    fn go_up(&mut self) {
        self.y -= 1;
    }

    fn go_left(&mut self) {
        self.x -= 1;
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
