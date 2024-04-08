#![allow(unused)]

use std::ops::{Add, Div, Mul, Sub};

use num::{Float, Num};

use super::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Clone> Vec4<T> {
    pub fn fill(f: T) -> Self {
        Self {
            x: f.clone(),
            y: f.clone(),
            z: f.clone(),
            w: f,
        }
    }

    pub fn xy(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }

    pub fn xyz(&self) -> Vec3<T> {
        Vec3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Clone + Num> Vec4<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x.clone() * rhs.x.clone()
            + self.y.clone() * rhs.y.clone()
            + self.z.clone() * rhs.z.clone()
            + self.w.clone() * rhs.w.clone()
    }

    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec4<T> {
    pub fn magnitude(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    pub fn angle(&self, rhs: &Self) -> T {
        (self.dot(rhs) / self.magnitude() * rhs.magnitude()).acos()
    }
}

impl<T: Num> From<[T; 3]> for Vec4<T> {
    fn from(values: [T; 3]) -> Self {
        let [x, y, z] = values;
        let one = T::one();
        Vec4::new(x, y, z, one)
    }
}

impl<T: Default> Default for Vec4<T> {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec4::new(
            self.x * rhs.clone(),
            self.y * rhs.clone(),
            self.z * rhs.clone(),
            self.w * rhs,
        )
    }
}

impl<T: Clone + Div<Output = T>> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Vec4::new(
            self.x / rhs.clone(),
            self.y / rhs.clone(),
            self.z / rhs.clone(),
            self.w / rhs,
        )
    }
}

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}
