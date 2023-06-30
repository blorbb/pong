use super::Color;

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.value
    }
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        Self {
            value: (r << 16) | (g << 8) | (b),
        }
    }

    pub const fn value(&self) -> u32 {
        self.value
    }
}
