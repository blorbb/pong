use super::base::Position;

pub struct Rectangle {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

mod rectangle;

pub struct Circle {
    radius: f64,
    center: Position,
}

mod circle;
