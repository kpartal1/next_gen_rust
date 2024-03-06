#![allow(unused)]

use std::iter;
use std::ops::{Add, Div, Mul, Sub};

use super::vec::Vec4;

#[derive(Debug)]
pub struct Mat4x4 {
    buf: [f32; 16],
}

impl Mat4x4 {
    pub fn new() -> Self {
        Mat4x4 { buf: [0.; 16] }
    }

    pub fn identity() -> Self {
        #[rustfmt::skip]
        let ident = [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ];
        Mat4x4 { buf: ident }
    }

    pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        #[rustfmt::skip]
        let proj = [
            2. * n / (r - l), 0.,               (r + l) / (r - l),  0.,
            0.,               2. * n / (t - b), (t + b) / (t - b),  0.,
            0.,               0.,               -(f + n) / (f - n), -(2. * f * n) / (f - n),
            0.,               0.,               -1.,                0.,
        ];
        Mat4x4 { buf: proj }
    }

    // This function is broken I think, if anyone can fix it I believe in you
    pub fn perspective(fov_y: f32, aspect: f32, front: f32, back: f32) -> Self {
        let tangent = 1. / (fov_y / 2.).to_radians().tan();
        let height = front * tangent;
        let width = height * aspect;

        Self::frustum(-width, width, -height, height, front, back)
    }

    pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        #[rustfmt::skip]
        let proj = [
            2. / (r - l), 0.,           0.,            -(r + l) / (r - l),
            0.,           2. / (t - b), 0.,            -(t + b) / (t - b),
            0.,           0.,           -2. / (f - n), -(f + n) / (f - n),
            0.,           0.,           0.,            1.,
        ];
        Mat4x4 { buf: proj }
    }

    pub fn look_at(eye: Vec4, center: Vec4, up: Vec4) -> Self {
        let z = (&eye - &center).normalize();
        let x = up.cross(&z);
        let y = z.cross(&x).normalize();
        let x = x.normalize();
        let xs = x.coefs();
        let ys = y.coefs();
        let zs = z.coefs();
        #[rustfmt::skip]
        let buf = [
            xs[0],        ys[0],        zs[0],        0.,
            xs[1],        ys[1],        zs[1],        0.,
            xs[2],        ys[2],        zs[2],        0.,
            -x.dot(&eye), -y.dot(&eye), -z.dot(&eye), 1.,
        ];
        Mat4x4 { buf }
    }

    pub fn rows(&self) -> [f32; 16] {
        self.buf
    }

    pub fn columns(&self) -> [f32; 16] {
        let buf = self.buf;
        let mut cols = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                cols[i * 4 + j] = buf[j * 4 + i];
            }
        }
        cols
    }
}

impl Default for Mat4x4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Mat4x4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { buf }
    }
}

impl Sub for Mat4x4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { buf }
    }
}

impl Mul<f32> for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: f32) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .map(|a| rhs * a)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { buf }
    }
}

impl Div<f32> for Mat4x4 {
    type Output = Mat4x4;
    fn div(self, rhs: f32) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .map(|a| a / rhs)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { buf }
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let coefs = rhs.coefs();
        let vec = self
            .buf
            .chunks(4)
            .zip(iter::repeat(coefs))
            .map(|(row, coefs)| {
                coefs
                    .into_iter()
                    .zip(row)
                    .fold(0., |acc, (a, b)| acc + a * b)
            })
            .collect::<Vec<_>>();
        Vec4::from(vec[0], vec[1], vec[2], vec[3])
    }
}

impl Mul<Vec4> for &Mat4x4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let coefs = rhs.coefs();
        let vec = self
            .buf
            .chunks(4)
            .zip(iter::repeat(coefs))
            .map(|(row, coefs)| {
                coefs
                    .into_iter()
                    .zip(row)
                    .fold(0., |acc, (a, b)| acc + a * b)
            })
            .collect::<Vec<_>>();
        Vec4::from(vec[0], vec[1], vec[2], vec[3])
    }
}

impl Mul for Mat4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut buf = [0.; 16];
        for (i, row) in self.buf.chunks(4).enumerate() {
            for (j, col) in rhs.columns().chunks(4).enumerate() {
                buf[j * 4 + i] = row.iter().zip(col).fold(0., |acc, (a, b)| acc + a * b);
            }
        }
        Mat4x4 { buf }
    }
}
