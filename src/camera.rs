use crate::linalg::vec3::Vec3;

pub struct Camera<T> {
    eye: Vec3<T>,
    look: Vec3<T>,
    up: Vec3<T>,
}

impl<T> Camera<T> {
    pub fn new(eye: Vec3<T>, look: Vec3<T>, up: Vec3<T>) -> Self {
        Self { eye, look, up }
    }
}
