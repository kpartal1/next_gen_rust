use crate::linalg::{matrix::Mat4x4, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    eye: Vec3<f32>,
    center: Vec3<f32>,
    up: Vec3<f32>,
}

impl Camera {
    pub fn new(eye: Vec3<f32>, center: Vec3<f32>, up: Vec3<f32>) -> Self {
        Self { eye, center, up }
    }

    pub fn look_at(self) -> Mat4x4<f32> {
        Mat4x4::look_at(self.eye, self.center, self.up)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let default = Vec3::default();
        Self::new(default.clone(), default.clone(), default)
    }
}
