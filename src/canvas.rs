#![allow(unused)]

use crate::{buffer::Buffer, color::Color};

pub struct Canvas {
    buf: Buffer,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: Buffer::new(width, height),
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear(Color::WHITE);  
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buf.draw_pixel(x, y, color);
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        self.buf.draw_line(x0, y0, x1, y1, color);
    }

    pub fn buffer(&self) -> &[u32] {
        self.buf.data()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
