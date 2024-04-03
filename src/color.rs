#![allow(unused)]

use std::ops::{Deref, DerefMut};

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Color(u32);

impl Deref for Color {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Color {
    pub const BLACK: Color = Color(0);
    pub const WHITE: Color = Color(16777215);
    pub const GREEN: Color = Color(65280);
    pub const RED: Color = Color(16711680);
    pub const BLUE: Color = Color(255);

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> Color {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        Color((r << 16) | (g << 8) | b)
    }

    pub fn random() -> Color {
        Color(rand::random())
    }
}
