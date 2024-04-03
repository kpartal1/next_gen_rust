#![allow(unused)]

use std::ops::{Deref, DerefMut};

use crate::{buffer::Buffer, color::Color};

pub struct Canvas {
    buf: Buffer,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buf: Buffer::new(width, height),
        }
    }

    pub fn buffer(&self) -> &[Color] {
        &self.buf
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn fill(&mut self) {
        self.buf.fill();
    }

    pub fn set_color(&mut self, color: Color) {
        self.buf.set_color(color);
    }

    // fn point_to_pixel(&self, (x, y): (f32, f32)) -> (usize, usize) {
    //     let (width, height) = self.buf.size();
    //     let x = (x + 1.) / 2.;
    //     let y = (y + 1.) / 2.;
    //     (
    //         (x * (width - 1) as f32) as usize,
    //         (height - 1) - (y * (height - 1) as f32) as usize,
    //     )
    // }

    // pub fn pixel(&mut self, index: (f32, f32)) {
    //     self.buf.pixel(self.point_to_pixel(index));
    // }

    // pub fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) {
    //     self.buf
    //         .line(self.point_to_pixel(p1), self.point_to_pixel(p2))
    // }

    // pub fn tri(&mut self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) {
    //     self.buf.tri(
    //         self.point_to_pixel(p1),
    //         self.point_to_pixel(p2),
    //         self.point_to_pixel(p3),
    //     )
    // }

    // pub fn poly(&mut self, points: Vec<(f32, f32)>) {
    //     self.line(points[0], points[points.len() - 1]);
    //     points.into_iter().reduce(|p1, p2| {
    //         self.line(p1, p2);
    //         p2
    //     });
    // }
}
