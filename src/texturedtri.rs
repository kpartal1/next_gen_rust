use obj::TexturedVertex;

use crate::linalg::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct TexturedTri {
    p1: Vec3<f32>,
    p2: Vec3<f32>,
    p3: Vec3<f32>,
    n1: Vec3<f32>,
    n2: Vec3<f32>,
    n3: Vec3<f32>,
    t1: Vec3<f32>,
    t2: Vec3<f32>,
    t3: Vec3<f32>,
}

impl From<[TexturedVertex; 3]> for TexturedTri {
    fn from([v1, v2, v3]: [TexturedVertex; 3]) -> Self {
        Self {
            p1: Vec3::from(v1.position),
            p2: Vec3::from(v2.position),
            p3: Vec3::from(v3.position),
            n1: Vec3::from(v1.normal),
            n2: Vec3::from(v2.normal),
            n3: Vec3::from(v3.normal),
            t1: Vec3::from(v1.texture),
            t2: Vec3::from(v2.texture),
            t3: Vec3::from(v3.texture),
        }
    }
}
