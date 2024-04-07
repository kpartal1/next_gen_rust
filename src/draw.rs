use crate::canvas::Canvas;

pub trait Draw {
    fn draw(&self, buf: &mut Canvas);
}
