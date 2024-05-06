#![allow(unused)]

use crate::{buffer2::Buffer2, color2::Color2};

pub struct Canvas3 {
    buf: Buffer2,
    width: usize,
    height: usize,
}

impl Canvas3 {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: Buffer2::new(width, height),
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear(Color2::WHITE);  
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color2) {
        self.buf.draw_pixel(x, y, color);
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color2) {
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