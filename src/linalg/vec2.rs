#![allow(unused)]

use std::ops::{Add, Mul, Sub};

use num::{Float, Num};

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Clone> Vec2<T> {
    pub fn fill(f: T) -> Self {
        Self { x: f.clone(), y: f }
    }

    pub fn x(&self) -> T {
        self.x.clone()
    }

    pub fn y(&self) -> T {
        self.y.clone()
    }
}

impl<T: Clone + Num> Vec2<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x.clone() * rhs.x.clone() + self.y.clone() * rhs.y.clone()
    }

    pub fn magnitude2(&self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec2<T> {
    pub fn magnitude(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
        }
    }

    pub fn angle(&self, rhs: &Self) -> T {
        (self.dot(rhs) / self.magnitude() * rhs.magnitude()).acos()
    }
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec2::new(self.x * rhs.clone(), self.y * rhs)
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
