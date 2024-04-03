mod buffer;
mod camera;
mod canvas;
mod color;
mod draw;
mod linalg;
mod renderer;
mod tri;
mod wireframe;

use buffer::Buffer;
use color::Color;
use linalg::{matrix::Mat4x4, vec3::Vec3, vec4::Vec4};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let eye = Vec3::new(0., 0., 1.);
    let center = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let view = Mat4x4::look_at(eye, center, up);
    println!("{view:?}");
    // let mut buffer = Buffer::new(WIDTH, HEIGHT);

    // let mut window = Window::new(
    //     "Test - ESC to exit",
    //     WIDTH,
    //     HEIGHT,
    //     WindowOptions {
    //         resize: true,
    //         ..Default::default()
    //     },
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // // Don't limit fps update rate
    // // window.limit_update_rate(None);

    // // All of this stuff is slightly wrong :)
    // let model = Mat4x4::identity();
    // let eye = Vec4::new(0., 0., 1., 1.);
    // let center = Vec4::new(0., 0., 0., 1.);
    // let up = Vec4::new(0., 1., 0., 1.);
    // let view = Mat4x4::look_at(eye.xyz(), center.xyz(), up.xyz());
    // let projection = Mat4x4::perspective(45., WIDTH as f32 / HEIGHT as f32, 0.1, 100.);
    // // let projection = Mat4x4::frustum(-1., 1., -1., 1., 1., 100.);
    // // let projection = Mat4x4::ortho(-1., 1., -1., 1., 1., 100.);
    // let mut mvp = projection * view * model;

    // let mut rng = rand::thread_rng();
    // let angle = 0.05;
    // let scale = 1.001;
    // // mvp.translate(1., 0., 0.);
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     buffer.clear();
    //     // canvas.set_color(Color::random());
    //     // mvp.rotate_y(angle);
    //     // mvp.rotate_x(angle);
    //     // mvp.scale(0.975, 1.025, 1.);
    //     mvp.rotate_z(angle);
    //     // let x1 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     // let y1 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     // let x2 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     // let y2 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     // let x3 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     // let y3 = rng.gen_range(-1f32..1. + f32::EPSILON);
    //     let p1 @ (x1, y1) = (mvp.clone() * Vec4::new(0., 1., 0., 1.)).xy();
    //     let p2 @ (x2, y2) = (mvp.clone() * Vec4::new(-1., -1., 0., 1.)).xy();
    //     let p3 @ (x3, y3) = (mvp.clone() * Vec4::new(1., -1., 0., 1.)).xy();
    //     // let p4 @ (x4, y4) = (mvp.clone() * Vec4::new(1., 1., -1., 0.)).xy();
    //     buffer.tri(p1, p2, p3);

    //     window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
    // }
}
