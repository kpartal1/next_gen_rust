#![allow(unused)]
use crate::{linalg::vec2::Vec2, Color};
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Canvas {
    buf: Vec<u32>,
    width: usize,
    height: usize,
}

impl<I: Into<(usize, usize)>> Index<I> for Canvas {
    type Output = u32;
    fn index(&self, index: I) -> &Self::Output {
        let (x, y) = index.into();
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &self.buf[(y * self.width) + x]
    }
}

impl<I: Into<(usize, usize)>> IndexMut<I> for Canvas {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let (x, y) = index.into();
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &mut self.buf[(y * self.width) + x]
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: vec![Color::BLACK; width * height],
            width,
            height,
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

    pub fn buffer(&self) -> &[u32] {
        &self.buf
    }

    // pub fn color(&self) -> u32 {
    //     self.color
    // }

    // pub fn set_color(&mut self, color: u32) {
    //     self.color = color;
    // }

    // pub fn fill(&mut self) {
    //     let color = self.color;
    //     self.buf.iter_mut().for_each(|x| *x = color);
    // }

    pub fn clear(&mut self) {
        self.buf.iter_mut().for_each(|x| *x = Color::BLACK);
    }

    pub fn pixel(&mut self, (x, y): (i32, i32), color: u32) {
        let (w, h) = (self.width as i32, self.height as i32);
        if x >= -w / 2 && x < w / 2 && y >= -h / 2 && y < h / 2 {
            let (x, y) = ((w / 2 + x) as usize, (h / 2 - y - 1) as usize);
            self[(x, y)] = color;
        }
    }

    fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
        if i0 == i1 {
            return vec![d0];
        }
        let mut values = Vec::with_capacity((i1 - i0) as usize);
        let a = (d1 - d0) as f32 / (i1 - i0) as f32;
        let mut d = d0 as f32;
        for i in i0..i1 {
            values.push(d as i32);
            d += a;
        }
        values
    }

    pub fn line(&mut self, (x0, y0): (i32, i32), (x1, y1): (i32, i32), color: u32) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let (mut x, mut y) = (x0, y0);
        let mut error = dx + dy;
        loop {
            self.pixel((x, y), color);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x == x1 {
                    break;
                }
                error += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y1 {
                    break;
                }
                error += dx;
                y += sy;
            }
        }
    }

    pub fn wireframe_tri(&mut self, p0: (i32, i32), p1: (i32, i32), p2: (i32, i32), color: u32) {
        self.line(p0, p1, color);
        self.line(p1, p2, color);
        self.line(p2, p0, color);
    }

    fn fill_bottom_flat_tri(
        &mut self,
        (x0, y0): (i32, i32),
        (x1, y1): (i32, i32),
        (x2, y2): (i32, i32),
        color: u32,
    ) {
        let dx1 = (x1 - x0) as f32 / (y1 - y0) as f32;
        let dx2 = (x2 - x0) as f32 / (y2 - y0) as f32;

        let mut cx1 = x0 as f32;
        let mut cx2 = x0 as f32;

        for scan_y in y0..=y1 {
            let c1 = cx1 as i32;
            let c2 = cx2 as i32;
            for x in i32::min(c1, c2)..i32::max(c1, c2) {
                self.pixel((x, scan_y), color);
            }
            // self.line((cx1 as i32, scan_y), (cx2 as i32, scan_y), color);
            cx1 += dx1;
            cx2 += dx2;
        }
    }

    fn fill_top_flat_tri(
        &mut self,
        (x0, y0): (i32, i32),
        (x1, y1): (i32, i32),
        (x2, y2): (i32, i32),
        color: u32,
    ) {
        let dx1 = (x2 - x0) as f32 / (y2 - y0) as f32;
        let dx2 = (x2 - x1) as f32 / (y2 - y1) as f32;

        let mut cx1 = x2 as f32;
        let mut cx2 = x2 as f32;

        let mut scan_y = y2;
        while scan_y > y0 {
            let c1 = cx1 as i32;
            let c2 = cx2 as i32;
            for x in i32::min(c1, c2)..i32::max(c1, c2) {
                self.pixel((x, scan_y), color);
            }
            // self.line((cx1 as i32, scan_y), (cx2 as i32, scan_y), color);
            cx1 -= dx1;
            cx2 -= dx2;
            scan_y -= 1;
        }
    }

    pub fn tri(
        &mut self,
        (mut x0, mut y0): (i32, i32),
        (mut x1, mut y1): (i32, i32),
        (mut x2, mut y2): (i32, i32),
        color: u32,
    ) {
        if y1 < y0 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        if y2 < y0 {
            std::mem::swap(&mut x0, &mut x2);
            std::mem::swap(&mut y0, &mut y2);
        }
        if y2 < y1 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }
        let mut x01 = Self::interpolate(y0, x0, y1, x1);
        let mut x12 = Self::interpolate(y1, x1, y2, x2);
        let x02 = Self::interpolate(y0, x0, y2, x2);

        // x01.pop();
        x01.append(&mut x12);
        let x012 = x01;

        let m = x012.len() / 2;
        let (mut x_left, mut x_right);
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        } else {
            x_left = x012;
            x_right = x02;
        }

        for y in y0..y2 {
            for x in x_left[(y - y0) as usize]..x_right[(y - y0) as usize] {
                self.pixel((x, y), color);
            }
        }
    }
}
