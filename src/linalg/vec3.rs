#![allow(unused)]

use std::ops::{Add, Mul, Sub};

use num::{Float, Num};

#[derive(Clone, Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Clone> Vec3<T> {
    pub fn fill(f: T) -> Self {
        Self {
            x: f.clone(),
            y: f.clone(),
            z: f,
        }
    }

    pub fn x(&self) -> T {
        self.x.clone()
    }

    pub fn y(&self) -> T {
        self.y.clone()
    }

    pub fn z(&self) -> T {
        self.z.clone()
    }
}

impl<T: Clone + Num> Vec3<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x.clone() * rhs.x.clone()
            + self.y.clone() * rhs.y.clone()
            + self.z.clone() * rhs.z.clone()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z.clone(),
            z: self.x.clone() * rhs.y.clone() - self.y.clone() * rhs.x.clone(),
        }
    }

    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec3<T> {
    pub fn magnitude(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }

    pub fn angle(&self, rhs: &Self) -> T {
        (self.dot(rhs) / self.magnitude() * rhs.magnitude()).acos()
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
