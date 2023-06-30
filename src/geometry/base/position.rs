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
}
