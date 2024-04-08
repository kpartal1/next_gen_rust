mod camera;
mod canvas;
mod color;
mod draw;
mod linalg;
mod renderer;
mod wireframe;

use std::{fs::File, io::BufReader};

use canvas::Canvas;
use color::Color;
use linalg::{matrix::Mat4x4, vec3::Vec3, vec4::Vec4};
use minifb::{Key, Window, WindowOptions};
use obj::{Obj, TexturedVertex};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let monkey = File::open("src/resources/monkey.obj").unwrap();
    let monkey: Obj<TexturedVertex, usize> = obj::load_obj(BufReader::new(monkey)).unwrap();
    println!("{monkey:?}");
    // let eye = Vec3::new(0., 0., 1.);
    // let center = Vec3::new(0., 0., 0.);
    // let up = Vec3::new(0., 1., 0.);
    // let view = Mat4x4::look_at(eye, center, up);
    // println!("{view:?}");
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

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // // Don't limit fps update rate
    // // window.limit_update_rate(None);

    // // All of this stuff is slightly wrong :)
    let model = Mat4x4::identity();
    let eye = Vec3::new(0., 0., 1.);
    let center = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let view = Mat4x4::look_at(eye, center, up);
    let projection = Mat4x4::perspective(45., WIDTH as f32 / HEIGHT as f32, 0.1, 100.);
    // let projection = Mat4x4::frustum(-1., 1., -1., 1., 1., 100.);
    // let projection = Mat4x4::ortho(-1., 1., -1., 1., 1., 100.);
    let mut mvp = projection * view * model;

    // let mut rng = rand::thread_rng();
    let angle = 0.01;
    let scale = 60.;
    println!("{mvp:?}");
    mvp.scale(scale, 40., 1.);
    println!("{mvp:?}");
    // mvp.translate(0., -1.5, 0.);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        canvas.clear();
        // canvas.set_color(Color::random());
        mvp.rotate_y(angle);
        // mvp.rotate_x(angle);
        // mvp.scale(0.975, 1.025, 1.);
        // mvp.rotate_z(angle);
        // let x1 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let y1 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let x2 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let y2 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let x3 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let y3 = rng.gen_range(-1f32..1. + f32::EPSILON);
        // let p1 @ (x1, y1) = (mvp.clone() * Vec4::new(0., 50., 0., 1.)).xy();
        // let p2 @ (x2, y2) = (mvp.clone() * Vec4::new(-50., -50., 0., 1.)).xy();
        // let p3 @ (x3, y3) = (mvp.clone() * Vec4::new(50., -50., 0., 1.)).xy();
        // let p4 @ (x4, y4) = (mvp.clone() * Vec4::new(1., 1., -1., 0.)).xy();
        // println!("{p1:?} {p2:?} {p3:?}");
        // canvas.tri(
        //     (x1 as i32, y1 as i32),
        //     (x2 as i32, y2 as i32),
        //     (x3 as i32, y3 as i32),
        //     Color::WHITE,
        // );
        let m = &monkey.vertices;
        for vs in monkey.indices.as_slice().chunks(3) {
            match vs {
                [v1, v2, v3] => {
                    let v1 = mvp.clone() * Vec4::from(m[*v1].position);
                    let v2 = mvp.clone() * Vec4::from(m[*v2].position);
                    let v3 = mvp.clone() * Vec4::from(m[*v3].position);
                    canvas.wireframe_tri(
                        (v1.x as i32, v1.y as i32),
                        (v2.x as i32, v2.y as i32),
                        (v3.x as i32, v3.y as i32),
                        Color::WHITE,
                    )
                }
                _ => unreachable!("The object loader would've caught this."),
            }
        }
        // let (w, h) = (WIDTH as i32, HEIGHT as i32);
        // canvas.pixel(-w / 2, h / 2 - 1, Color::WHITE);
        // canvas.line((0, 0), (100, -30), Color::BLUE);
        // canvas.tri((-200, -250), (200, 50), (20, 250), Color::WHITE);

        window
            .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
