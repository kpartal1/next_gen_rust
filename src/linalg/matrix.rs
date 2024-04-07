#![allow(unused)]

use num::{Float, Num};
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

use super::vec3::Vec3;
use super::vec4::Vec4;

#[derive(Clone, PartialEq)]
pub struct Mat4x4<T> {
    c1: Vec4<T>,
    c2: Vec4<T>,
    c3: Vec4<T>,
    c4: Vec4<T>,
}

impl<T: Clone + Num> Mat4x4<T> {
    pub fn new() -> Self {
        let c = Vec4::new(T::zero(), T::zero(), T::zero(), T::zero());
        Mat4x4 {
            c1: c.clone(),
            c2: c.clone(),
            c3: c.clone(),
            c4: c,
        }
    }

    pub fn from_cols(c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>, c4: Vec4<T>) -> Self {
        Mat4x4 { c1, c2, c3, c4 }
    }

    pub fn cols(self) -> [Vec4<T>; 4] {
        [
            self.c1.clone(),
            self.c2.clone(),
            self.c3.clone(),
            self.c4.clone(),
        ]
    }

    pub fn rows(self) -> [Vec4<T>; 4] {
        let r1 = Vec4::new(
            self.c1.x.clone(),
            self.c2.x.clone(),
            self.c3.x.clone(),
            self.c4.x.clone(),
        );
        let r2 = Vec4::new(
            self.c1.y.clone(),
            self.c2.y.clone(),
            self.c3.y.clone(),
            self.c4.y.clone(),
        );
        let r3 = Vec4::new(
            self.c1.z.clone(),
            self.c2.z.clone(),
            self.c3.z.clone(),
            self.c4.z.clone(),
        );
        let r4 = Vec4::new(
            self.c1.w.clone(),
            self.c2.w.clone(),
            self.c3.w.clone(),
            self.c4.w.clone(),
        );
        [r1, r2, r3, r4]
    }

    pub fn identity() -> Self {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(one.clone(), zero.clone(), zero.clone(), zero.clone());
        let c2 = Vec4::new(zero.clone(), one.clone(), zero.clone(), zero.clone());
        let c3 = Vec4::new(zero.clone(), zero.clone(), one.clone(), zero.clone());
        let c4 = Vec4::new(zero.clone(), zero.clone(), zero.clone(), one.clone());
        Mat4x4 { c1, c2, c3, c4 }
    }

    pub fn translate(&mut self, tx: T, ty: T, tz: T) {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(one.clone(), zero.clone(), zero.clone(), zero.clone());
        let c2 = Vec4::new(zero.clone(), one.clone(), zero.clone(), zero.clone());
        let c3 = Vec4::new(zero.clone(), zero.clone(), one.clone(), zero.clone());
        let c4 = Vec4::new(tx, ty, tz, one.clone());
        let trans = Mat4x4 { c1, c2, c3, c4 };
        *self = self.clone() * trans;
    }

    pub fn scale(&mut self, sx: T, sy: T, sz: T) {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(sx, zero.clone(), zero.clone(), zero.clone());
        let c2 = Vec4::new(zero.clone(), sy, zero.clone(), zero.clone());
        let c3 = Vec4::new(zero.clone(), zero.clone(), sz, zero.clone());
        let c4 = Vec4::new(zero.clone(), zero.clone(), zero.clone(), one.clone());
        let scale = Mat4x4 { c1, c2, c3, c4 };
        *self = self.clone() * scale;
    }
}

impl<T: Float> Mat4x4<T> {
    pub fn rotate_x(&mut self, t: T) {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(one, zero, zero, zero);
        let c2 = Vec4::new(zero, t.cos(), -t.sin(), zero);
        let c3 = Vec4::new(zero, t.sin(), t.cos(), zero);
        let c4 = Vec4::new(zero, zero, zero, one);
        let rot = Mat4x4 { c1, c2, c3, c4 };
        *self = self.clone() * rot;
    }

    pub fn rotate_y(&mut self, t: T) {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(t.cos(), zero, t.sin(), zero);
        let c2 = Vec4::new(zero, one, zero, zero);
        let c3 = Vec4::new(-t.sin(), zero, t.cos(), zero);
        let c4 = Vec4::new(zero, zero, zero, one);
        let rot = Mat4x4 { c1, c2, c3, c4 };
        *self = self.clone() * rot;
    }

    pub fn rotate_z(&mut self, t: T) {
        let zero = T::zero();
        let one = T::one();
        let c1 = Vec4::new(t.cos(), t.sin(), zero, zero);
        let c2 = Vec4::new(-t.sin(), t.cos(), zero, zero);
        let c3 = Vec4::new(zero, zero, one, zero);
        let c4 = Vec4::new(zero, zero, zero, one);
        let rot = Mat4x4 { c1, c2, c3, c4 };
        *self = self.clone() * rot;
    }

    pub fn frustum(l: T, r: T, b: T, t: T, n: T, f: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let c1 = Vec4::new(two * n / (r - l), zero, zero, zero);
        let c2 = Vec4::new(zero, two * n / (t - b), zero, zero);
        let c3 = Vec4::new(
            (r + l) / (r - l),
            (t + b) / (t - b),
            -(f + n) / (f - n),
            -one,
        );
        let c4 = Vec4::new(zero, zero, -(two * f * n) / (f - n), zero);
        Mat4x4 { c1, c2, c3, c4 }
    }

    pub fn perspective(fov_y: T, aspect: T, front: T, back: T) -> Self {
        let one = T::one();
        let two = one + one;
        let tangent = one / (fov_y / two).to_radians().tan() * front;
        let height = front * tangent;
        let width = height * aspect;

        Self::frustum(-width, width, -height, height, front, back)
    }

    pub fn ortho(l: T, r: T, b: T, t: T, n: T, f: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let c1 = Vec4::new(two / (r - l), zero, zero, zero);
        let c2 = Vec4::new(zero, two / (t - b), zero, zero);
        let c3 = Vec4::new(zero, zero, -two / (f - n), zero);
        let c4 = Vec4::new(
            -(r + l) / (r - l),
            -(t + b) / (t - b),
            -(f + n) / (f - n),
            one,
        );
        Mat4x4 { c1, c2, c3, c4 }
    }

    pub fn look_at(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self {
        let zero = T::zero();
        let one = T::one();
        let z = (eye.clone() - center).normalize();
        let x = up.cross(&z);
        let y = z.cross(&x).normalize();
        let x = x.normalize();
        let c1 = Vec4::new(x.x, x.y, x.z, -x.dot(&eye));
        let c2 = Vec4::new(y.x, y.y, y.z, -y.dot(&eye));
        let c3 = Vec4::new(z.x, z.y, z.y, -z.dot(&eye));
        let c4 = Vec4::new(zero, zero, zero, one);
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Default> Default for Mat4x4<T> {
    fn default() -> Self {
        let c1 = Vec4::default();
        let c2 = Vec4::default();
        let c3 = Vec4::default();
        let c4 = Vec4::default();
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Num> Add for Mat4x4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let c1 = self.c1 + rhs.c1;
        let c2 = self.c2 + rhs.c2;
        let c3 = self.c3 + rhs.c3;
        let c4 = self.c4 + rhs.c4;
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Num> Sub for Mat4x4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let c1 = self.c1 - rhs.c1;
        let c2 = self.c2 - rhs.c2;
        let c3 = self.c3 - rhs.c3;
        let c4 = self.c4 - rhs.c4;
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Clone + Num> Mul<T> for Mat4x4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let c1 = self.c1 * rhs.clone();
        let c2 = self.c2 * rhs.clone();
        let c3 = self.c3 * rhs.clone();
        let c4 = self.c4 * rhs;
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Clone + Num> Div<T> for Mat4x4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let c1 = self.c1 / rhs.clone();
        let c2 = self.c2 / rhs.clone();
        let c3 = self.c3 / rhs.clone();
        let c4 = self.c4 / rhs;
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: Clone + Num> Mul<Vec4<T>> for Mat4x4<T> {
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let [r1, r2, r3, r4] = self.rows();
        Vec4::new(r1.dot(&rhs), r2.dot(&rhs), r3.dot(&rhs), r4.dot(&rhs))
    }
}

impl<T: Clone + Num> Mul for Mat4x4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let [c1, c2, c3, c4] = rhs.cols();
        let c1 = self.clone() * c1;
        let c2 = self.clone() * c2;
        let c3 = self.clone() * c3;
        let c4 = self * c4;
        Mat4x4 { c1, c2, c3, c4 }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Mat4x4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "|{:?} {:?} {:?} {:?}|\n|{:?} {:?} {:?} {:?}|\n|{:?} {:?} {:?} {:?}|\n|{:?} {:?} {:?} {:?}|",
            self.c1.x, self.c2.x, self.c3.x, self.c4.x, self.c1.y, self.c2.y, self.c3.y, self.c4.y, self.c1.z, self.c2.z, self.c3.z, self.c4.z, self.c1.w, self.c2.w, self.c3.w, self.c4.w
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::linalg::vec4::Vec4;

    use super::Mat4x4;

    #[test]
    fn matadd() {
        let mut c1 = Vec4::new(1, 5, 9, 13);
        let mut c2 = Vec4::new(2, 6, 10, 14);
        let mut c3 = Vec4::new(3, 7, 11, 15);
        let mut c4 = Vec4::new(4, 8, 12, 16);
        let m1 = Mat4x4::from_cols(c1.clone(), c2.clone(), c3.clone(), c4.clone());
        let m2 = m1.clone();
        c1 = c1 * 2;
        c2 = c2 * 2;
        c3 = c3 * 2;
        c4 = c4 * 2;
        assert_eq!(m1 + m2, Mat4x4::from_cols(c1, c2, c3, c4));
    }

    #[test]
    fn matsub() {
        let c1 = Vec4::new(1, 5, 9, 13);
        let c2 = Vec4::new(2, 6, 10, 14);
        let c3 = Vec4::new(3, 7, 11, 15);
        let c4 = Vec4::new(4, 8, 12, 16);
        let m1 = Mat4x4::from_cols(c1, c2, c3, c4);
        let m2 = m1.clone();
        assert_eq!(m1 - m2, Mat4x4::default());
    }

    #[test]
    fn matmul() {
        let c1 = Vec4::new(1, 5, 9, 13);
        let c2 = Vec4::new(2, 6, 10, 14);
        let c3 = Vec4::new(3, 7, 11, 15);
        let c4 = Vec4::new(4, 8, 12, 16);
        let m1 = Mat4x4::from_cols(c1, c2, c3, c4);
        let m2 = m1.clone();
        let c1 = Vec4::new(90, 202, 314, 426);
        let c2 = Vec4::new(100, 228, 356, 484);
        let c3 = Vec4::new(110, 254, 398, 542);
        let c4 = Vec4::new(120, 280, 440, 600);
        assert_eq!(m1 * m2, Mat4x4::from_cols(c1, c2, c3, c4));
    }

    #[test]
    fn vecmatmul() {
        let c1 = Vec4::new(1, 5, 9, 13);
        let c2 = Vec4::new(2, 6, 10, 14);
        let c3 = Vec4::new(3, 7, 11, 15);
        let c4 = Vec4::new(4, 8, 12, 16);
        let m1 = Mat4x4::from_cols(c1, c2, c3, c4);
        let v1 = Vec4::new(3, 5, 1, 4);
        assert_eq!(m1 * v1, Vec4::new(32, 84, 136, 188));
    }
}
