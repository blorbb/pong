use crate::display::{Color, Screen};

pub trait Draw {
    fn draw_on(&self, screen: &mut Screen, color: Color);
}

pub struct Rectangle {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

mod rectangle;
