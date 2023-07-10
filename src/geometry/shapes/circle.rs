use crate::{
    display::{Color, Screen},
    geometry::{
        base::Position,
        traits::{Contains, Coords, Draw},
    },
};

use super::Circle;

impl Contains<Position> for Circle {
    fn contains(&self, other: Position) -> bool {
        let dist2 = self.center.dist2(other);
        dist2 <= self.radius.powi(2)
    }
}

impl Coords for Circle {
    fn coords(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        let (minx, maxx) = (self.center.x() - self.radius, self.center.x() + self.radius);
        let (miny, maxy) = (self.center.y() - self.radius, self.center.y() + self.radius);

        let x_range = minx.ceil() as i64..maxx.floor() as i64;
        let y_range = miny.ceil() as i64..maxy.floor() as i64;

        y_range.flat_map(move |y| {
            x_range.clone().filter_map(move |x| {
                if self.contains(Position::new(x as f64, y as f64)) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }
}

impl Draw for Circle {
    fn draw_on(&self, screen: &mut Screen, color: &Color) {
        let x_left = (self.center.x() - self.radius).ceil() as usize;
        let x_right = (self.center.x() + self.radius).floor() as usize;
        let y_top = (self.center.y() - self.radius).ceil() as usize;
        let y_bottom = (self.center.y() + self.radius).floor() as usize;

        let x_range = x_left.max(0)..x_right.min(screen.width() - 1);
        let y_range = y_top.max(0)..y_bottom.min(screen.height() - 1);

        for y in y_range {
            let mut intersected_once = false;
            for x in x_range.clone() {
                if self.contains(Position::new(x as f64, y as f64)) {
                    screen.set_pixel(x, y, color);
                    intersected_once = true;
                } else if intersected_once {
                    // has intersected once and pixel is outside circle now
                    // there will be no more intersections for the rest of the row
                    break;
                };
            }
        }
    }
}

impl Circle {
    pub fn from_center_radius(center: Position, radius: f64) -> Circle {
        Self { radius, center }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> &Position {
        &self.center
    }
}
