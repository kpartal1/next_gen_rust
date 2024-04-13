use obj::TexturedVertex;

use crate::linalg::vec3::Vec3;

pub struct Tri {
    p1: Vec3<f32>,
    p2: Vec3<f32>,
    p3: Vec3<f32>,
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
