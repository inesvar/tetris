//! Defines a point on a grid that can move.
use serde::{Deserialize, Serialize};

/// Point on a finite wrap-around 2D grid.
///
/// A point moves without knownledge of its surroundings through [Transform]
/// and thus doesn't implement collisions.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(super) struct Point {
    /// horizontal coordinate, from left to right
    pub(super) x: i8,
    /// vertical coordinate, *from top to bottom*
    pub(super) y: i8,
}

/// Unhindered moves on a grid (used when collisions aren't necessary)
///
/// ## Uses
/// - [Tetromino](super::Tetromino) *center* (go_down, go_right, go_left)
/// - [Tetromino](super::Tetromino) *blocks* (rotate_clockwise, rotate_counterclockwise) when checking if a rotation is possible
/// - [TetrisGrid](super::TetrisGrid) *blocks* (go_down, go_up) when receiving/completing lines
pub(super) trait Transform {
    /// Move down one cell without checking if it's empty.
    fn go_down(&mut self);
    /// Move up one cell without checking if it's empty.
    fn go_up(&mut self);
    /// Move one cell left without checking if it's empty.
    fn go_left(&mut self);
    /// Move one cell right without checking if it's empty.
    fn go_right(&mut self);
    /// Rotate 90° clockwise around the given origin without checking if the destination is empty.
    fn rotate_clockwise(&mut self, other: &Point);
    /// Rotate 90° counterclockwise around the given origin without checking if the destination is empty.
    fn rotate_counterclockwise(&mut self, other: &Point);
}

impl Point {
    pub(super) fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, -10)
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
