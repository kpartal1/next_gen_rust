use minifb::Window;

use crate::canvas::Canvas;

pub struct WireFrame {
    window: Window,
    buffer: Canvas,
}
