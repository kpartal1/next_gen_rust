use minifb::{Key, Window, WindowOptions};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    draw::Draw,
    linalg::vec3::Vec3,
    renderer::{Projection, Renderer},
    wireframemodel::WireFrameModel,
};

pub struct WireFrameRenderer {
    window: Window,
    canvas: Canvas,
    camera: Camera,
    projection: Projection,
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
        let eye = Vec3::new(0., 0., 1.);
        let center = Vec3::new(0., 0., 0.);
        let up = Vec3::new(0., 1., 0.);
        let camera = Camera::new(eye, center, up);
        let projection = Projection::Perspective(45., width as f32 / height as f32, 0.1, 100.);
        let models = Vec::new();
        Self {
            window,
            canvas,
            camera,
            projection,
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
    fn render(&mut self) {
        let projection = self.projection.clone().mat();
        let view = self.camera.clone().look_at();
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.canvas.clear();
            for model in self.models.iter_mut() {
                model.model_mat().rotate_y(0.045);
                let mvp = projection.clone() * view.clone() * model.model_mat().clone();
                for tri in model.tris() {
                    let [p1, p2, p3] = tri.points();
                    let p1 = mvp.clone() * p1.clone();
                    let p2 = mvp.clone() * p2.clone();
                    let p3 = mvp.clone() * p3.clone();
                    self.canvas.wireframe_tri(
                        (p1.x as i32, p1.y as i32),
                        (p2.x as i32, p2.y as i32),
                        (p3.x as i32, p3.y as i32),
                        Color::WHITE,
                    )
                }
                model.draw(&mut self.canvas);
            }

            self.window
                .update_with_buffer(
                    self.canvas.buffer(),
                    self.canvas.width(),
                    self.canvas.height(),
                )
                .unwrap();
        }
    }
}
