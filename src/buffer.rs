#![allow(unused)]
use crate::{linalg::vec2::Vec2, Color};
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Buffer {
    buf: Vec<Color>,
    width: usize,
    height: usize,
    color: Color,
}

impl Deref for Buffer {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

impl Index<Vec2<usize>> for Buffer {
    type Output = Color;
    fn index(&self, Vec2 { x, y }: Vec2<usize>) -> &Self::Output {
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &self.buf[(y * self.width) + x]
    }
}

impl IndexMut<Vec2<usize>> for Buffer {
    fn index_mut(&mut self, Vec2 { x, y }: Vec2<usize>) -> &mut Self::Output {
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &mut self.buf[(y * self.width) + x]
    }
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: vec![Color::BLACK; width * height],
            width,
            height,
            color: Color::WHITE,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn clear(&mut self) {
        self.iter_mut().for_each(|x| *x = Color::BLACK);
    }

    pub fn fill(&mut self) {
        let color = self.color;
        self.iter_mut().for_each(|x| *x = color);
    }

    pub fn pixel(&mut self, index: Vec2<usize>) {
        self[index] = self.color;
    }

    pub fn line(&mut self, Vec2 { x: x1, y: y1 }: Vec2<usize>, Vec2 { x: x2, y: y2 }: Vec2<usize>) {
        let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let (mut x, mut y) = (x1, y1);
        let mut error = dx + dy;
        loop {
            self.pixel(Vec2::new(x as usize, y as usize));
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                error += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                error += dx;
                y += sy;
            }
        }
    }

    pub fn tri(&mut self, p1: Vec2<usize>, p2: Vec2<usize>, p3: Vec2<usize>) {
        self.line(p1, p2);
        self.line(p2, p3);
        self.line(p3, p1);
    }
}
