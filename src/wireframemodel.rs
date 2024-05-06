use obj::{Obj, TexturedVertex};

use crate::{draw::Draw, linalg::matrix::Mat4x4, tri::Tri};

pub struct WireFrameModel {
    name: Option<String>,
    tris: Vec<Tri>,
    mat: Mat4x4<f32>,
}

impl WireFrameModel {
    pub fn model_mat(&mut self) -> &mut Mat4x4<f32> {
        &mut self.mat
    }

    pub fn tris(&mut self) -> &mut Vec<Tri> {
        &mut self.tris
    }
}

impl Draw for WireFrameModel {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        for tri in &self.tris {
            tri.draw(canvas);
        }
    }
}

impl From<Obj<TexturedVertex, usize>> for WireFrameModel {
    fn from(value: Obj<TexturedVertex, usize>) -> Self {
        let m = &value.vertices;
        let tris = value
            .indices
            .chunks_exact(3)
            .map(|vs| match vs {
                &[v1, v2, v3] => {
                    let vs = [m[v1], m[v2], m[v3]];
                    Tri::from(vs)
                }
                _ => unreachable!("Model must be triangulated."),
            })
            .collect();
        Self {
            name: value.name,
            tris,
            mat: Mat4x4::identity(),
        }
    }
}
