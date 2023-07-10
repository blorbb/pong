use crate::{
    display::{Color, Screen},
    geometry::{
        base::Position,
        traits::{Contains, Coords, Draw},
    },
};

use super::Rectangle;

impl Contains<Position> for Rectangle {
    fn contains(&self, other: Position) -> bool {
        let (x, y) = (other.x(), other.y());
        self.x1 < x && x < self.x2 && self.y1 < y && y < self.y2
    }
}

impl Coords for Rectangle {
    fn coords(&self) -> impl Iterator<Item = (i64, i64)> {
        let x_range = (self.x1.ceil() as i64)..(self.x2.floor() as i64);
        let y_range = (self.y1.ceil() as i64)..(self.y2.floor() as i64);
        y_range.flat_map(move |y| x_range.clone().map(move |x| (x, y)))
    }
}

impl Draw for Rectangle {
    fn draw_on(&self, screen: &mut Screen, color: &Color) {
        let x_range =
            (self.x1.ceil() as usize).max(0)..(self.x2.floor() as usize).min(screen.width() - 1);
        let y_range =
            (self.y1.ceil() as usize).max(0)..(self.y2.floor() as usize).min(screen.height() - 1);
        for x in x_range {
            for y in y_range.clone() {
                screen.set_pixel(x, y, color);
            }
        }
    }
}

impl Rectangle {
    pub fn from_corners(top_left: Position, bottom_right: Position) -> Self {
        let xs = [top_left.x(), bottom_right.x()];
        let ys = [top_left.y(), bottom_right.y()];
        Self {
            x1: f64::min(xs[0], xs[1]),
            y1: f64::min(ys[0], ys[1]),
            x2: f64::max(xs[0], xs[1]),
            y2: f64::max(ys[0], ys[1]),
        }
    }
}
