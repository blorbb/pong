use softbuffer::Buffer;

use crate::geometry::shapes::Draw;

use super::{Color, Screen};

impl<'a> Screen<'a> {
    pub fn new(width: usize, height: usize, buffer: Buffer<'a>) -> Self {
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Sets the pixel at the given coordinates.
    /// Does not check that the coordinates given are in the correct range,
    /// and panics if trying to index out of range.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[x + y * self.width] = color.value();
    }

    /// Tries to set a pixel at a specified coordinate.
    ///
    /// Returns `true` if it was successfully set, otherwise returns `false`.
    pub fn try_set_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            self.set_pixel(x, y, color);
            true
        }
    }

    pub fn present(self) {
        self.buffer.present().expect("Failed to present buffer");
    }

    pub fn draw<T: Draw>(&mut self, shape: T, color: Color) {
        shape.draw_on(self, color)
    }
}
