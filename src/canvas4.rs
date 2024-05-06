#![allow(unused)]

use std::ops::{Deref, DerefMut};

use num::{abs, integer::{self, sqrt}};

use crate::{buffer::Buffer, color::Color};

pub struct Canvas4 {
    buf: Buffer,
}

impl Canvas4 {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas4 {
            buf: Buffer::new(width, height),
        }
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buf
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn fill(&mut self) {
        self.buf.fill();
    }

    pub fn set_color(&mut self, color: u32) {
        self.buf.set_color(color);
    }

    fn point_to_pixel(&self, (x, y): (f32, f32)) -> (usize, usize) {
        let (width, height) = self.buf.size();
        let x = (x + 1.) / 2.;
        let y = (y + 1.) / 2.;
        (
            (x * (width - 1) as f32) as usize,
            (height - 1) - (y * (height - 1) as f32) as usize,
        )
    }

    pub fn pixel(&mut self, index: (f32, f32)) {
        self.buf.pixel(self.point_to_pixel(index));
    }

    pub fn projection_2_d(cam_pos: &[f32; 3], point: (f32, f32, f32)) -> (f32, f32) {
        let dist: isize = sqrt(((cam_pos[0] - point.0)*(cam_pos[0] - point.0) + (cam_pos[1] - point.1)*(cam_pos[1] - point.1) + (cam_pos[2] - point.2)*(cam_pos[2] - point.2)) as isize);
        let cam_dist: isize = sqrt(((cam_pos[0])*(cam_pos[0]) + (cam_pos[1])*(cam_pos[1]) + (cam_pos[2])*(cam_pos[2])) as isize);
        let proj_point:[f32; 2] = [point.0*(cam_dist as f32)/(dist as f32)/100.0, point.1*(cam_dist as f32)/(dist as f32)/100.0];
        (proj_point[0], proj_point[1])
    }

    pub fn pixel_3_d(&mut self, cam_pos: &[f32; 3], index: (f32, f32, f32)) {
        let proj_point = Self::projection_2_d(cam_pos, index);
        self.buf.pixel(self.point_to_pixel(proj_point));
    }

    pub fn line(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        self.buf
            .line(self.point_to_pixel(p1), self.point_to_pixel(p2))
    }

    pub fn line_3_d(&mut self, cam_pos: &[f32; 3], p1: (f32, f32, f32), p2: (f32, f32, f32)) {
        let proj_p1 = Self::projection_2_d(cam_pos, p1);
        let proj_p2 = Self::projection_2_d(cam_pos, p2);
        self.buf
            .line(self.point_to_pixel(proj_p1), self.point_to_pixel(proj_p2))
    }

    pub fn tri(&mut self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) {
        self.buf.tri(
            self.point_to_pixel(p1),
            self.point_to_pixel(p2),
            self.point_to_pixel(p3),
        )
    }

    pub fn sqr(&mut self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), p4: (f32, f32)) {
        self.buf.sqr(
            self.point_to_pixel(p1),
            self.point_to_pixel(p2),
            self.point_to_pixel(p3),
            self.point_to_pixel(p4),
        )
    }

    pub fn sqr_3_d(&mut self, cam_pos: &[f32; 3], p1: (f32, f32, f32), p2: (f32, f32, f32), p3: (f32, f32, f32), p4: (f32, f32, f32)) {
        let proj_p1 = Self::projection_2_d(cam_pos, p1);
        let proj_p2 = Self::projection_2_d(cam_pos, p2);
        let proj_p3 = Self::projection_2_d(cam_pos, p3);
        let proj_p4 = Self::projection_2_d(cam_pos, p4);
        self.buf.sqr(
            self.point_to_pixel(proj_p1),
            self.point_to_pixel(proj_p2),
            self.point_to_pixel(proj_p3),
            self.point_to_pixel(proj_p4),
        )
    }

    pub fn cube(&mut self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), p4: (f32, f32), p5: (f32, f32), p6: (f32, f32), p7: (f32, f32), p8: (f32, f32)) {
        self.sqr(p1, p2, p3, p4);
        self.set_color(Color::random());
        self.sqr(p1, p2, p6, p5);
        self.set_color(Color::random());
        self.sqr(p2, p6, p7, p3);
        self.set_color(Color::random());
        self.sqr(p3, p4, p8, p7);
        self.set_color(Color::random());
        self.sqr(p1, p4, p8, p5);
    }
}