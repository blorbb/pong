use crate::display::{Color, Screen};

pub trait Contains<T> {
    /// Method defining whether some shape completely contains another shape/point.
    ///
    /// Shapes/points on the edge of `self` should be counted as contained.
    fn contains(&self, other: T) -> bool;
}

pub trait Coords {
    /// Returns an iterable of integer points that are contained within `self`.
    ///
    /// Coordinates on the edge of `self` should be counted.
    fn coords(&self) -> impl Iterator<Item = (i64, i64)> + '_;
}

pub trait Draw {
    /// Draws the shape on the screen in the specified color.
    ///
    /// Pixels not in the shape should not be modified.
    fn draw_on(&self, screen: &mut Screen, color: &Color);
}
