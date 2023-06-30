/// A basic 2D vector. Can be treated as anything (point, direction, normal).
///
/// Prefer to use the stricter types (Point, Vector and Normal).
#[derive(Debug, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

mod vec2;

/// A position in 2D space. Based on the Vec2 struct.
#[derive(Debug, PartialEq)]
pub struct Position {
    vec: Vec2,
}

mod position;
