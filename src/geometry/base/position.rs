use super::{Position, Vec2};

impl From<Vec2> for Position {
    fn from(value: Vec2) -> Self {
        Self { vec: value }
    }
}

impl From<Position> for Vec2 {
    fn from(value: Position) -> Self {
        value.vec
    }
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            vec: Vec2::new(x, y),
        }
    }

    pub const fn x(&self) -> f64 {
        self.vec.x
    }

    pub const fn y(&self) -> f64 {
        self.vec.y
    }

    /// Returns the **square** distance between two points.
    pub fn dist2(&self, other: Self) -> f64 {
        (self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)
    }

    /// Returns the distance between two points.
    ///
    /// If the square distance can be used instead, prefer `dist2`.
    pub fn dist(&self, other: Self) -> f64 {
        self.dist2(other).sqrt()
    }
}
