use crate::buffer::Buffer;

pub trait Draw {
    fn draw(&self, buf: &mut Buffer);
}
