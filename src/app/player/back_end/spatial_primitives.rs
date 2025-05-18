//! Define `struct` [Block] and `struct` [Position].
use super::{Deserialize, Serialize, TetrisColor};

/// Block in a discrete grid, serializable.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct Block {
    position: Position,
    color: TetrisColor,
}

/// Position on a discrete grid, serializable.
#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Debug)]
pub(super) struct Position {
    /// horizontal coordinate, from left to right
    pub(super) x: i32,
    /// vertical coordinate, *from top to bottom*
    pub(super) y: i32,
}

pub(super) const FALL: Position = Position::new(0, 1);
pub(super) const RISE: Position = Position::new(0, -1);
pub(super) const RIGHT: Position = Position::new(1, 0);
pub(super) const LEFT: Position = Position::new(-1, 0);

impl Block {
    // TODO remove if really unused
    pub(super) const fn new(color: TetrisColor, x: i32, y: i32) -> Self {
        Block {
            position: Position::new(x, y),
            color,
        }
    }

    pub(super) const fn from(color: TetrisColor, position: Position) -> Self {
        Block { position, color }
    }

    // This is used to delegate to the `position` field.
    pub(super) fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    // Is this really useful ?
    pub(super) fn x(&self) -> i32 {
        self.position.x
    }

    pub(super) fn y(&self) -> i32 {
        self.position.y
    }

    pub(super) fn color(&self) -> TetrisColor {
        self.color
    }
}

impl Position {
    pub(super) const fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    // Using `const` is useful here so that well-kicks can be evaluated at compile-time in the future.
    // TODO create a const trait Arithmetic when it will be possible
    pub(super) const fn neg(self) -> Self {
        Position::new(-self.x, -self.y)
    }

    pub(super) const fn mirror_x(self) -> Self {
        Position::new(-self.x, self.y)
    }

    pub(super) const fn mirror_y(self) -> Self {
        Position::new(self.x, -self.y)
    }

    pub(super) const fn add(self, other: Position) -> Self {
        Position::new(self.x + other.x, self.y + other.y)
    }

    pub(super) const fn turned_clockwise(&self) -> Self {
        Position::new(-self.y, self.x)
    }

    pub(super) const fn turned_counterclockwise(&self) -> Self {
        Position::new(self.y, -self.x)
    }
}

// Needed to use a circular buffer.
impl Default for Block {
    fn default() -> Self {
        Block {
            position: Position::default(),
            color: TetrisColor::Yellow, // arbitrary
        }
    }
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Self::Output {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Self::Output {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn arithmetic_implementation_is_correct() {
        // y increases from top to bottom...
        let three_oclock = Position::new(1, 0);
        let six_oclock = Position::new(0, 1);
        let nine_oclock = Position::new(-1, 0);
        let twelve_oclock = Position::new(0, -1);

        assert_eq!(three_oclock.turned_clockwise(), six_oclock);
        assert_eq!(nine_oclock.turned_counterclockwise(), six_oclock);
        assert_eq!(twelve_oclock.neg(), six_oclock);
    }

    #[test]
    fn arithmetic_implementations_are_equivalent() {
        let a = Position::new(6, 3);
        let b = Position::new(5, 4);

        assert_eq!(a + b, a.add(b));
        assert_eq!(a - b, a.add(b.neg()));

        let mut sum = a;
        sum += b;

        assert_eq!(sum, a + b);
    }
}
