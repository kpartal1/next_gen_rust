#![allow(unused)]

use num::{Float, Num};
use std::iter;
use std::mem::MaybeUninit;
use std::ops::{Add, Div, Mul, Sub};

use super::vec3::Vec3;
use super::vec4::Vec4;

#[derive(Debug)]
pub struct Mat4x4<T> {
    buf: [T; 16],
}

impl<T> Mat4x4<T> {
    pub fn new(buf: [T; 16]) -> Self {
        Mat4x4 { buf }
    }
}

impl<T: Clone + Num> Mat4x4<T> {
    pub fn rows(&self) -> [T; 16] {
        self.buf.clone()
    }

    pub fn columns(&self) -> [T; 16] {
        let mut cols = self.buf.clone();
        for i in 0..4 {
            for j in 0..4 {
                cols[i * 4 + j] = self.buf[j * 4 + i].clone();
            }
        }
        cols
    }
}

impl<T: Float> Mat4x4<T> {
    pub fn identity() -> Self {
        let zero = T::zero();
        let one = T::one();
        #[rustfmt::skip]
        let ident = [
            one, zero, zero, zero,
            zero, one, zero, zero,
            zero, zero, one, zero,
            zero, zero, zero, one,
        ];
        Mat4x4 { buf: ident }
    }

    pub fn frustum(l: T, r: T, b: T, t: T, n: T, f: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        #[rustfmt::skip]
        let proj = [
            two * n / (r - l), zero,              (r + l) / (r - l),        zero,
            zero,              two * n / (t - b), (t + b) / (t - b),        zero,
            zero,              zero,              -one * (f + n) / (f - n), -(two * f * n) / (f - n),
            zero,              zero,              -one,                     zero,
        ];
        Mat4x4 { buf: proj }
    }

    // This function is broken I think, if anyone can fix it I believe in you
    pub fn perspective(fov_y: T, aspect: T, front: T, back: T) -> Self {
        let one = T::one();
        let two = one + one;
        let tangent = one / (fov_y / two).to_radians().tan();
        let height = front * tangent;
        let width = height * aspect;

        Self::frustum(-width, width, -height, height, front, back)
    }

    pub fn ortho(l: T, r: T, b: T, t: T, n: T, f: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        #[rustfmt::skip]
        let proj = [
            two / (r - l), zero,           zero,            -(r + l) / (r - l),
            zero,           two / (t - b), zero,            -(t + b) / (t - b),
            zero,           zero,           -two / (f - n), -(f + n) / (f - n),
            zero,           zero,           zero,            one,
        ];
        Mat4x4 { buf: proj }
    }

    pub fn look_at(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self {
        let zero = T::zero();
        let one = T::one();
        let z = (eye.clone() - center).normalize();
        let x = up.cross(&z);
        let y = z.cross(&x).normalize();
        let x = x.normalize();
        #[rustfmt::skip]
        let buf = [
            x.x(),        y.x(),        z.x(),        zero,
            x.y(),        y.y(),        z.y(),        zero,
            x.z(),        y.z(),        z.z(),        zero,
            -x.dot(&eye), -y.dot(&eye), -z.dot(&eye), one,
        ];
        Mat4x4 { buf }
    }
}

impl<T: Default> Default for Mat4x4<T> {
    fn default() -> Self {
        Self::new([
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ])
    }
}

impl<T: Num> Add for Mat4x4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();
        Self { buf }
    }
}

impl<T: Num> Sub for Mat4x4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .zip(rhs.buf)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();
        Self { buf }
    }
}

impl<T: Clone + Num> Mul<T> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .map(|a| rhs.clone() * a)
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();
        Self { buf }
    }
}

impl<T: Clone + Num> Div<T> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn div(self, rhs: T) -> Self::Output {
        let buf = self
            .buf
            .into_iter()
            .map(|a| a / rhs.clone())
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();
        Self { buf }
    }
}

impl<T: Clone + Num> Mul<Vec4<T>> for Mat4x4<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let mut new_coefs = [T::zero(), T::zero(), T::zero(), T::zero()];
        let mut count = 0;
        for i in 0..4 {
            new_coefs[count] = self.buf[i].clone() * rhs.x()
                + self.buf[i + 1].clone() * rhs.y()
                + self.buf[i + 2].clone() * rhs.z()
                + self.buf[i + 3].clone() * rhs.w();
            count += 1;
        }
        Vec4::new(
            new_coefs[0].clone(),
            new_coefs[1].clone(),
            new_coefs[2].clone(),
            new_coefs[3].clone(),
        )
    }
}

impl<T: Clone + Num> Mul<Vec4<T>> for &Mat4x4<T> {
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let mut new_coefs = [T::zero(), T::zero(), T::zero(), T::zero()];
        let mut count = 0;
        for i in 0..4 {
            new_coefs[count] = self.buf[i].clone() * rhs.x()
                + self.buf[i + 1].clone() * rhs.y()
                + self.buf[i + 2].clone() * rhs.z()
                + self.buf[i + 3].clone() * rhs.w();
            count += 1;
        }
        Vec4::new(
            new_coefs[0].clone(),
            new_coefs[1].clone(),
            new_coefs[2].clone(),
            new_coefs[3].clone(),
        )
    }
}

impl<T: Clone + Num> Mul for Mat4x4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut buf = self.rows();
        for (i, row) in self.buf.chunks(4).enumerate() {
            for (j, col) in rhs.columns().chunks(4).enumerate() {
                buf[j * 4 + i] = row
                    .iter()
                    .zip(col)
                    .fold(T::zero(), |acc, (a, b)| acc + a.clone() * b.clone());
            }
        }
        Mat4x4 { buf }
    }
}
