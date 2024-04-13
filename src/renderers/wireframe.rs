use minifb::{Window, WindowOptions};

use crate::{canvas::Canvas, renderer::Renderer, wireframemodel::WireFrameModel};

pub struct WireFrameRenderer {
    window: Window,
    canvas: Canvas,
    models: Vec<WireFrameModel>,
}

impl WireFrameRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions {
                resize: true,
                ..Default::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        let canvas = Canvas::new(width, height);
        let models = Vec::new();
        Self {
            window,
            canvas,
            models,
        }
    }

    pub fn add_models(&mut self, models: Vec<WireFrameModel>) {
        self.models.extend(models);
    }

    pub fn add_model(&mut self, model: WireFrameModel) {
        self.models.push(model);
    }
}

impl Renderer for WireFrameRenderer {
    fn render(&mut self) {}
}
