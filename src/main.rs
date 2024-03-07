mod buffer;
mod canvas;
mod color;
mod linalg;

use canvas::Canvas;
use color::Color;
use linalg::vec2::Vec2;
use linalg::{matrix::Mat4x4, vec4::Vec4};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // All of this stuff is slightly wrong :)
    // let model = Mat4x4::identity();
    // let eye = Vec4::from(0., 0., 1., 0.);
    // let center = Vec4::new();
    // let up = Vec4::from(0., 1., 0., 0.);
    // let view = Mat4x4::look_at(eye, center, up);
    let projection = Mat4x4::perspective(45., WIDTH as f32 / HEIGHT as f32, 0.1, 100.);
    println!("{:?}", projection.rows());
    println!("{:?}", projection.columns());
    // let projection = Mat4x4::frustum(-1., 1., -1., 1., 1., 100.);
    // let mvp = projection * view * model;
    let v = Vec2::new(1., 1.);
    let m = v.magnitude();

    let mut rng = rand::thread_rng();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        canvas.clear();
        canvas.set_color(Color::random());
        let x1 = rng.gen_range(-1f32..1. + f32::EPSILON);
        let y1 = rng.gen_range(-1f32..1. + f32::EPSILON);
        let x2 = rng.gen_range(-1f32..1. + f32::EPSILON);
        let y2 = rng.gen_range(-1f32..1. + f32::EPSILON);
        let x3 = rng.gen_range(-1f32..1. + f32::EPSILON);
        let y3 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let (x1, y1) = (&mvp * Vec4::from(0., 0.5, 0., 0.)).xy();
        // let (x2, y2) = (&mvp * Vec4::from(-0.5, -0.5, 0., 0.)).xy();
        // let (x3, y3) = (&mvp * Vec4::from(0.5, -0.5, 0., 0.)).xy();
        canvas.tri((x1, y1), (x2, y2), (x3, y3));

        window
            .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
