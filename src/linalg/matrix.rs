#![allow(unused)]

use std::iter;
use std::ops::{Add, Div, Mul, Sub};

use super::vec::Vec4;

pub struct Mat4x4 {
    buf: [f32; 16],
}

impl Mat4x4 {
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

    pub fn projection(r: f32, t: f32, n: f32, f: f32) -> Self {
        #[rustfmt::skip]
        let proj = [
            n / r, 0.,    0.,           0.,
            0.,    n / t, 0.,           0.,
            0.,    0.,    -f / (f - n), (-f * n) / (f - n),
            0.,    0.,    -1.,          0.,
        ];
        Mat4x4 { buf: proj }
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

impl Mul<u32> for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: u32) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .map(|a| rhs as f32 * a)
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

impl Mul for Mat4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let buf = self
            .buf
            .chunks(4)
            .zip(rhs.buf.chunks(4))
            .map(|(mat1, mat2)| mat1.iter().zip(mat2).fold(0., |acc, (a, b)| acc + a * b))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Mat4x4 { buf }
    }
}
