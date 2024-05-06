#![allow(unused)]

use num::Float;
use crate::linalg::vec3::Vec3; 

#[derive(Clone, Debug, PartialEq)]
pub struct Mat4x4<T> {
    buf: [T; 16],
}

impl<T: Float> Mat4x4<T> {
    pub fn new(buf: [T; 16]) -> Self {
        Mat4x4 { buf }
    }

    pub fn identity() -> Self {
        let zero = T::zero();
        let one = T::one();
        let ident = [
            one, zero, zero, zero,
            zero, one, zero, zero,
            zero, zero, one, zero,
            zero, zero, zero, one,
        ];
        Mat4x4 { buf: ident }
    }

    pub fn rotation_y(angle: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let (sin, cos) = angle.sin_cos();
        let rotation = [
            cos, zero, sin, zero,
            zero, one, zero, zero,
            -sin, zero, cos, zero,
            zero, zero, zero, one,
        ];
        Mat4x4 { buf: rotation }
    }

    pub fn transform_vector(&self, vec: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.buf[0] * vec.x + self.buf[4] * vec.y + self.buf[8] * vec.z,
            y: self.buf[1] * vec.x + self.buf[5] * vec.y + self.buf[9] * vec.z,
            z: self.buf[2] * vec.x + self.buf[6] * vec.y + self.buf[10] * vec.z,
        }
    }
}