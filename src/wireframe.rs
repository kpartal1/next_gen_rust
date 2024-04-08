use minifb::Window;
use obj::Obj;

use crate::{canvas::Canvas, renderer::Renderer};

pub struct WireFrame {
    window: Window,
    buffer: Canvas,
    meshes: Vec<Obj>,
}
