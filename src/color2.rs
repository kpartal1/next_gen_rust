#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Color2 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color2 {
    pub const BLACK: Color2 = Color2 { r: 0, g: 0, b: 0 };
    pub const WHITE: Color2 = Color2 { r: 255, g: 255, b: 255 };
    pub const RED: Color2 = Color2 { r: 255, g: 0, b: 0 };
    pub const GREEN: Color2 = Color2 { r: 0, g: 255, b: 0 };
    pub const BLUE: Color2 = Color2 { r: 0, g: 0, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Color2 {
        Color2 { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}