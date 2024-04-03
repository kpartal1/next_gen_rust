use crate::{draw::Draw, linalg::vec3::Vec3};

pub struct Tri<T> {
    p1: Vec3<T>,
    p2: Vec3<T>,
    p3: Vec3<T>,
}
