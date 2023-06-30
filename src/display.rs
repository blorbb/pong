use softbuffer::Buffer;

pub struct Screen<'a> {
    width: usize,
    height: usize,
    buffer: Buffer<'a>,
}

mod screen;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    value: u32,
}

mod color;
