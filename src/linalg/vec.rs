#![allow(unused)]

use std::ops::{Add, Div, Mul, Sub};

pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub fn new() -> Self {
        Vec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }

    pub fn fill(f: f32) -> Self {
        Vec4 {
            x: f,
            y: f,
            z: f,
            w: f,
        }
    }

    pub fn from(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        let x = self.x / m;
        let y = self.y / m;
        let z = self.z / m;
        let w = self.w / m;
        Vec4 { x, y, z, w }
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn magnitude2(&self) -> f32 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vec4 { x, y, z, w: self.w }
    }

    pub fn coefs(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Vec4 { x, y, z, w }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Vec4 { x, y, z, w }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = rhs * self.x;
        let y = rhs * self.y;
        let z = rhs * self.z;
        let w = rhs * self.w;
        Vec4 { x, y, z, w }
    }
}

impl Mul<u32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        let x = rhs as f32 * self.x;
        let y = rhs as f32 * self.y;
        let z = rhs as f32 * self.z;
        let w = rhs as f32 * self.w;
        Vec4 { x, y, z, w }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        let w = self.w / rhs;
        Vec4 { x, y, z, w }
    }
}
