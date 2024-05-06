mod camera;
mod canvas;
mod color;
mod draw;
mod linalg;
mod model;
mod renderer;
mod renderers;
mod texturedtri;
mod tri;
mod wireframemodel;

mod buffer;
mod canvas2;

use std::f32::consts::{PI, SQRT_2};

use canvas2::Canvas2;
use canvas::Canvas;
use color::Color;
use linalg::vec2::Vec2;
use linalg::{matrix::Mat4x4, vec4::Vec4};
use minifb::{Key, Window, WindowOptions};
use num::integer::sqrt;
use num::traits::real::Real;
use num::{abs, pow, Float};
use rand::Rng;

use std::{fs::File, io::BufReader};

use std::env;

use obj::{Obj, TexturedVertex};
use renderer::Renderer;
use renderers::wireframe::WireFrameRenderer;
use wireframemodel::WireFrameModel;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    // Limit to max ~60 fps update rate

    let mut rng = rand::thread_rng();

    if query == "og" {
        let mut canvas = Canvas2::new(WIDTH, HEIGHT);

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
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
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
    } else if query == "kon" {
      let mut wireframe = WireFrameRenderer::new(WIDTH, HEIGHT);
    let monkey = File::open("src/resources/monkey.obj").unwrap();
    let monkey: Obj<TexturedVertex, usize> = obj::load_obj(BufReader::new(monkey)).unwrap();
    let mut monkey = WireFrameModel::from(monkey);
    monkey.model_mat().scale(110., 80., 80.);
    wireframe.add_model(monkey);
    wireframe.render();
    // let monkey2 = File::open("src/resources/monkey.obj").unwrap();
    // let monkey2: Obj<TexturedVertex, usize> = obj::load_obj(BufReader::new(monkey2)).unwrap();
    // let monkey3 = File::open("src/resources/monkey.obj").unwrap();
    // let monkey3: Obj<TexturedVertex, usize> = obj::load_obj(BufReader::new(monkey3)).unwrap();
    // println!("{monkey:?}");
      // let mut model = Mat4x4::identity();
    // model.scale(60., 40., 40.);
    // // let mut model2 = Mat4x4::identity();
    // // model2.scale(40., 40., 40.);
    // // model2.translate(150., 10., 1.);
    // // let mut model3 = Mat4x4::identity();
    // // model3.scale(40., 40., 40.);
    // // model3.translate(-150., -10., 1.);
    // let eye = Vec3::new(0., 0., 1.);
    // let center = Vec3::new(0., 0., 0.);
    // let up = Vec3::new(0., 1., 0.);
    // let view = Mat4x4::look_at(eye, center, up);
    // let projection =
    //     Mat4x4::perspective(45f32.to_radians(), WIDTH as f32 / HEIGHT as f32, 0.1, 100.);
    // // let projection = Mat4x4::frustum(-1., 1., -1., 1., 1., 100.);
    // // let projection = Mat4x4::ortho(-1., 1., -1., 1., 1., 100.);
    // let mut mvp = projection.clone() * view.clone() * model;
    // // let mut mvp2 = projection.clone() * view.clone() * model2;
    // // println!("{:?}", Vec4::from(monkey2.vertices[0].position));
    // // println!(
    // //     "{:?}",
    // //     mvp2.clone() * Vec4::from(monkey2.vertices[0].position)
    // // );
    // // let mut mvp3 = projection * view * model3;
      // let angle = 0.045;
    // // let angle2 = 0.1;
    // // let angle3 = 0.01;
    // // mvp.scale(40., 40., 1.);
    // // mvp2.scale(60., 40., 1.);
    // // mvp3.scale(60., 40., 1.);
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     canvas.clear();
    //     // mvp.rotate_x(angle);
    //     mvp.rotate_y(angle);
    //     // mvp2.rotate_y(angle2);
    //     // mvp3.rotate_y(angle3);
    //     // mvp.rotate_z(angle);
    //     let m = &monkey.vertices;
    //     for vs in monkey.indices.chunks(3) {
    //         match vs {
    //             &[v1, v2, v3] => {
    //                 let v1 = mvp.clone() * Vec4::from(m[v1].position);
    //                 let v2 = mvp.clone() * Vec4::from(m[v2].position);
    //                 let v3 = mvp.clone() * Vec4::from(m[v3].position);
    //                 canvas.wireframe_tri(
    //                     (v1.x as i32, v1.y as i32),
    //                     (v2.x as i32, v2.y as i32),
    //                     (v3.x as i32, v3.y as i32),
    //                     Color::WHITE,
    //                 )
    //             }
    //             _ => unreachable!("The object loader would've caught this."),
    //         }
    //     }
    //     // let m = &monkey2.vertices;
    //     // for vs in monkey2.indices.chunks(3) {
    //     //     match vs {
    //     //         &[v1, v2, v3] => {
    //     //             let v1 = mvp2.clone() * Vec4::from(m[v1].position);
    //     //             let v2 = mvp2.clone() * Vec4::from(m[v2].position);
    //     //             let v3 = mvp2.clone() * Vec4::from(m[v3].position);
    //     //             canvas.wireframe_tri(
    //     //                 (v1.x as i32, v1.y as i32),
    //     //                 (v2.x as i32, v2.y as i32),
    //     //                 (v3.x as i32, v3.y as i32),
    //     //                 Color::WHITE,
    //     //             )
    //     //         }
    //     //         _ => unreachable!("The object loader would've caught this."),
    //     //     }
    //     // }
    //     // let m = &monkey3.vertices;
    //     // for vs in monkey3.indices.chunks(3) {
    //     //     match vs {
    //     //         &[v1, v2, v3] => {
    //     //             let v1 = mvp3.clone() * Vec4::from(m[v1].position);
    //     //             let v2 = mvp3.clone() * Vec4::from(m[v2].position);
    //     //             let v3 = mvp3.clone() * Vec4::from(m[v3].position);
    //     //             canvas.wireframe_tri(
    //     //                 (v1.x as i32, v1.y as i32),
    //     //                 (v2.x as i32, v2.y as i32),
    //     //                 (v3.x as i32, v3.y as i32),
    //     //                 Color::WHITE,
    //     //             )
    //     //         }
    //     //         _ => unreachable!("The object loader would've caught this."),
    //     //     }
    //     // }
    //     // let x = mvp.clone() * Vec4::new(1000., 0., 0., 0.);
    //     // let xn = mvp.clone() * Vec4::new(-1000., 0., 0., 0.);
    //     // let y = mvp.clone() * Vec4::new(0., 1000., 0., 0.);
    //     // let yn = mvp.clone() * Vec4::new(0., -1000., 0., 0.);
    //     // let z = mvp.clone() * Vec4::new(0., 0., 1000., 0.);
    //     // let zn = mvp.clone() * Vec4::new(0., 0., -1000., 0.);
    //     // canvas.line(
    //     //     (xn.x as i32, xn.y as i32),
    //     //     (x.x as i32, x.y as i32),
    //     //     Color::RED,
    //     // );
    //     // canvas.line(
    //     //     (yn.x as i32, yn.y as i32),
    //     //     (y.x as i32, y.y as i32),
    //     //     Color::GREEN,
    //     // );
    //     // canvas.line(
    //     //     (zn.x as i32, zn.y as i32),
    //     //     (z.x as i32, z.y as i32),
    //     //     Color::BLUE,
    //     // );

    //     window
    //         .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
    //         .unwrap();
    // }
  }
}
