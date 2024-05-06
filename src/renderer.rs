use crate::linalg::matrix::Mat4x4;

#[derive(Clone, Debug)]
pub enum Projection {
    Perspective(f32, f32, f32, f32),
    Orthogonal,
    Custom(Mat4x4<f32>),
}

impl Projection {
    pub fn mat(self) -> Mat4x4<f32> {
        match self {
            Self::Perspective(fovy, aspect, near, far) => {
                Mat4x4::perspective(fovy, aspect, near, far)
            }
            Self::Orthogonal => Mat4x4::ortho(-1., 1., -1., 1., -1., 1.),
            Self::Custom(m) => m,
        }
    }
}

pub trait Renderer {
    fn render(&mut self);
}
