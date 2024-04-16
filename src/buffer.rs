#![allow(unused)]
use crate::color::Color;

pub struct Buffer {
    buf: Vec<u32>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.buf.fill(color.to_u32()); 
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buf[y * self.width + x] = color.to_u32();
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let mut x = x0 as i32;
        let mut y = y0 as i32;
        let dx = (x1 as i32 - x0 as i32).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 as i32 - y0 as i32).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        while {
            self.draw_pixel(x as usize, y as usize, color);
            x != x1 as i32 || y != y1 as i32
        } {
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn data(&self) -> &[u32] {
        &self.buf
    }
}
