use obj::{Obj, TexturedVertex};

use crate::{linalg::matrix::Mat4x4, tri::Tri};

pub struct WireFrameModel {
    name: Option<String>,
    tris: Vec<Tri>,
    mat: Mat4x4<f32>,
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
