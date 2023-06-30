use crate::{
    display::{Color, Screen},
    geometry::base::Position,
};

use super::{Draw, Rectangle};

impl Draw for Rectangle {
    fn draw_on(&self, screen: &mut Screen, color: Color) {
        for (x, y) in self.coords() {
            screen.try_set_pixel(x as usize, y as usize, color);
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

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        self.x1 < x && x < self.x2 && self.y1 < y && y < self.y2
    }

    pub fn coords(&self) -> impl Iterator<Item = (i64, i64)> {
        let x_range = (self.x1.ceil() as i64)..(self.x2.floor() as i64);
        let y_range = (self.y1.ceil() as i64)..(self.y2.floor() as i64);
        y_range.flat_map(move |y| x_range.clone().map(move |x| (x, y)))
    }
}
