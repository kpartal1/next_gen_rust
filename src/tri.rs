use obj::TexturedVertex;

use crate::{color::Color, draw::Draw, linalg::vec3::Vec3};

pub struct Tri {
    p1: Vec3<f32>,
    p2: Vec3<f32>,
    p3: Vec3<f32>,
}

impl Tri {
    pub fn points(&mut self) -> [&mut Vec3<f32>; 3] {
        [&mut self.p1, &mut self.p2, &mut self.p3]
    }
}

impl Draw for Tri {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        canvas.tri(
            (self.p1.x as i32, self.p1.y as i32),
            (self.p2.x as i32, self.p2.y as i32),
            (self.p3.x as i32, self.p3.y as i32),
            Color::WHITE,
        );
    }
}

impl From<[TexturedVertex; 3]> for Tri {
    fn from([v1, v2, v3]: [TexturedVertex; 3]) -> Self {
        Self {
            p1: Vec3::from(v1.position),
            p2: Vec3::from(v2.position),
            p3: Vec3::from(v3.position),
        }
    }
}
