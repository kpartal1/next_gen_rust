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
mod canvas3;
mod color2;
mod buffer2;
use std::{thread, time};
mod canvas4;


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
  } else if query == "trevor" {
    use linalg::vec3::Vec3;
    use linalg::matrix2::Mat4x4;
    use color2::Color2;
    use canvas3::Canvas3;

    struct Cube {
        vertices: [Vec3<f32>; 8],
    }
    
    impl Cube {
        fn new() -> Self {
            Cube {
                vertices: [
                    Vec3::new(-1.0, -1.0, -1.0),
                    Vec3::new(1.0, -1.0, -1.0),
                    Vec3::new(1.0, 1.0, -1.0),
                    Vec3::new(-1.0, 1.0, -1.0),
                    Vec3::new(-1.0, -1.0, 1.0),
                    Vec3::new(1.0, -1.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(-1.0, 1.0, 1.0),
                ],
            }
        }
    
        fn rotate(&mut self, angle: f32, _axis: Vec3<f32>) {  
            let rotation_matrix = Mat4x4::rotation_y(angle);
            for vertex in &mut self.vertices {
                *vertex = rotation_matrix.transform_vector(vertex.clone());
            }
        }
        
    }
    
    fn draw_cube(canvas: &mut Canvas3, cube: &Cube, color: Color2) {
        for i in 0..cube.vertices.len() {
            let start = &cube.vertices[i];
            let end = &cube.vertices[(i + 1) % cube.vertices.len()];
            let (sx, sy) = project_to_screen(start.clone());
            let (ex, ey) = project_to_screen(end.clone());
            canvas.draw_line(sx, sy, ex, ey, color); 
        }
    }
    
    
    fn project_to_screen(vertex: Vec3<f32>) -> (usize, usize) {
        let scale = 100.0; 
        let screen_width = 800.0;  
        let screen_height = 600.0;
        let x = ((vertex.x * scale) + (screen_width / 2.0)) as usize;
        let y = ((-vertex.z * scale) + (screen_height / 2.0)) as usize;  
        (x, y)
    }

    let mut window = minifb::Window::new(
        "Test - ESC to exit",
        800,
        600,
        minifb::WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    let mut canvas = Canvas3::new(800, 600);
    let mut cube = Cube::new(); 

    let mut angle = 0.0;
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let axis = Vec3::new(0.0, 1.0, 0.0);  
        angle += PI / 180.0;  
        cube.rotate(angle, axis);

        canvas.clear();
        draw_cube(&mut canvas, &cube, Color2::BLUE);
        thread::sleep(time::Duration::from_millis(1));
        draw_cube(&mut canvas, &cube, Color2::GREEN);
        thread::sleep(time::Duration::from_millis(3));
        draw_cube(&mut canvas, &cube, Color2::BLACK);
        thread::sleep(time::Duration::from_millis(4));
        draw_cube(&mut canvas, &cube, Color2::RED);
  


        window.update_with_buffer(canvas.buffer(), canvas.width(), canvas.height()).unwrap();
    }
    } else if query == "ty" {
        use crate::canvas4::Canvas4;
        let mut canvas = Canvas4::new(WIDTH, HEIGHT);
        struct point {
            x: f32,
            y: f32,
            z: f32
        }
        
        struct face {
            point1: point,
            point2: point,
            point3: point,
            point4: point
        }
        
        struct cube {
            face1: face,
            face2: face,
            face3: face,
            face4: face,
            face5: face,
            face6: face
        }
        
        impl cube {
            pub fn new(point1: point, point2: point, point3: point, point4: point, point5: point, point6: point, point7: point, point8: point) -> cube {
                let face1 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point2.x, y: point2.y, z: point2.z}, point3: point{x: point3.x, y: point3.y, z: point3.z}, point4: point{x: point4.x, y: point4.y, z: point4.z}};
                let face2 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point2.x, y: point2.y, z: point2.z}, point3: point{x: point6.x, y: point6.y, z: point6.z}, point4: point{x: point5.x, y: point5.y, z: point5.z}};
                let face3 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point4.x, y: point4.y, z: point4.z}, point3: point{x: point8.x, y: point8.y, z: point8.z}, point4: point{x: point5.x, y: point5.y, z: point5.z}};
                let face4 = face{point1: point{x: point5.x, y: point5.y, z: point5.z}, point2: point{x: point6.x, y: point6.y, z: point6.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point8.x, y: point8.y, z: point8.z}};
                let face5 = face{point1: point{x: point4.x, y: point4.y, z: point4.z}, point2: point{x: point3.x, y: point3.y, z: point3.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point8.x, y: point8.y, z: point8.z}};
                let face6 = face{point1: point{x: point2.x, y: point2.y, z: point2.z}, point2: point{x: point3.x, y: point3.y, z: point3.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point6.x, y: point6.y, z: point6.z}};
                cube{face1: face1, face2: face2, face3: face3, face4: face4, face5: face5, face6: face6}
            }
        }
        
        const WIDTH: usize = 640;
        const HEIGHT: usize = 360;

        fn rotate_y(point: (f32, f32, f32), center: &(f32, f32, f32), rot_speed: f32) -> (f32, f32, f32){
            let x_len = point.0 - center.0;
            let z_len = point.2 - center.2;
            let mut angle = x_len.atan2(z_len);
            let y_dist = (x_len*x_len + z_len*z_len).sqrt() ;
            angle += rot_speed;
            let r1 = y_dist * angle.sin() + center.0;
            let r2 = y_dist * angle.cos() + center.0;
            (r1, point.1, r2)
        }
        
        fn rotate_x(point: (f32, f32, f32), center: &(f32, f32, f32), rot_speed: f32) -> (f32, f32, f32){
            let y_len = point.1 - center.1;
            let z_len = point.2 - center.2;
            let mut angle = y_len.atan2(z_len);
            let x_dist = (y_len*y_len + z_len*z_len).sqrt() ;
            angle += rot_speed;
            let r1 = x_dist * angle.sin() + center.0;
            let r2 = x_dist * angle.cos() + center.0;
            (point.0, r1, r2)
        }
        
        fn rotate_z(point: (f32, f32, f32), center: &(f32, f32, f32), rot_speed: f32) -> (f32, f32, f32){
            let y_len = point.1 - center.1;
            let x_len = point.0 - center.0;
            let mut angle = x_len.atan2(y_len);
            let z_dist = (y_len*y_len + x_len*x_len).sqrt() ;
            angle += rot_speed;
            let r1 = z_dist * angle.sin() + center.0;
            let r2 = z_dist * angle.cos() + center.0;
            (r1, r2, point.2)
        }
        
        fn make_cube(point1: point, point2: point, point3: point, point4: point, point5: point, point6: point, point7: point, point8: point) -> cube {
            let face1 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point2.x, y: point2.y, z: point2.z}, point3: point{x: point3.x, y: point3.y, z: point3.z}, point4: point{x: point4.x, y: point4.y, z: point4.z}};
            let face2 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point2.x, y: point2.y, z: point2.z}, point3: point{x: point6.x, y: point6.y, z: point6.z}, point4: point{x: point5.x, y: point5.y, z: point5.z}};
            let face3 = face{point1: point{x: point1.x, y: point1.y, z: point1.z}, point2: point{x: point4.x, y: point4.y, z: point4.z}, point3: point{x: point8.x, y: point8.y, z: point8.z}, point4: point{x: point5.x, y: point5.y, z: point5.z}};
            let face4 = face{point1: point{x: point5.x, y: point5.y, z: point5.z}, point2: point{x: point6.x, y: point6.y, z: point6.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point8.x, y: point8.y, z: point8.z}};
            let face5 = face{point1: point{x: point4.x, y: point4.y, z: point4.z}, point2: point{x: point3.x, y: point3.y, z: point3.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point8.x, y: point8.y, z: point8.z}};
            let face6 = face{point1: point{x: point2.x, y: point2.y, z: point2.z}, point2: point{x: point3.x, y: point3.y, z: point3.z}, point3: point{x: point7.x, y: point7.y, z: point7.z}, point4: point{x: point6.x, y: point6.y, z: point6.z}};
            cube{face1: face1, face2: face2, face3: face3, face4: face4, face5: face5, face6: face6}
        }
        
        fn show_cube(cube: &cube, cam_pos: &[f32; 3], canvas: &mut Canvas4) {
            canvas.sqr_3_d(cam_pos, (cube.face1.point1.x, cube.face1.point1.y, cube.face1.point1.z), (cube.face1.point2.x, cube.face1.point2.y, cube.face1.point2.z), (cube.face1.point3.x, cube.face1.point3.y, cube.face1.point3.z), (cube.face1.point4.x, cube.face1.point4.y, cube.face1.point4.z));
            //canvas.sqr_3_d(cam_pos, (cube.face2.point1.x, cube.face2.point1.y, cube.face2.point1.z), (cube.face2.point2.x, cube.face2.point2.y, cube.face2.point2.z), (cube.face2.point3.x, cube.face2.point3.y, cube.face3.point3.z), (cube.face2.point4.x, cube.face2.point4.y, cube.face2.point4.z));
            canvas.sqr_3_d(cam_pos, (cube.face3.point1.x, cube.face3.point1.y, cube.face3.point1.z), (cube.face3.point2.x, cube.face3.point2.y, cube.face3.point2.z), (cube.face3.point3.x, cube.face3.point3.y, cube.face3.point3.z), (cube.face3.point4.x, cube.face3.point4.y, cube.face3.point4.z));
            canvas.sqr_3_d(cam_pos, (cube.face4.point1.x, cube.face4.point1.y, cube.face4.point1.z), (cube.face4.point2.x, cube.face4.point2.y, cube.face4.point2.z), (cube.face4.point3.x, cube.face4.point3.y, cube.face4.point3.z), (cube.face4.point4.x, cube.face4.point4.y, cube.face4.point4.z));
            canvas.sqr_3_d(cam_pos, (cube.face5.point1.x, cube.face5.point1.y, cube.face5.point1.z), (cube.face5.point2.x, cube.face5.point2.y, cube.face5.point2.z), (cube.face5.point3.x, cube.face5.point3.y, cube.face5.point3.z), (cube.face5.point4.x, cube.face5.point4.y, cube.face5.point4.z));
            canvas.sqr_3_d(cam_pos, (cube.face6.point1.x, cube.face6.point1.y, cube.face6.point1.z), (cube.face6.point2.x, cube.face6.point2.y, cube.face6.point2.z), (cube.face6.point3.x, cube.face6.point3.y, cube.face6.point3.z), (cube.face6.point4.x, cube.face6.point4.y, cube.face6.point4.z));
        }
        
        fn rotate_cube_x(cube: cube, center: &(f32, f32, f32), rot_speed: f32) -> cube{
            let mut point1 = (cube.face1.point1.x, cube.face1.point1.y, cube.face1.point1.z);
            point1 = rotate_x(point1, center, rot_speed);
            let mut point2 = (cube.face1.point2.x, cube.face1.point2.y, cube.face1.point2.z);
            point2 = rotate_x(point2, center, rot_speed);
            let mut point3 = (cube.face1.point3.x, cube.face1.point3.y, cube.face1.point3.z);
            point3 = rotate_x(point3, center, rot_speed);
            let mut point4 = (cube.face1.point4.x, cube.face1.point4.y, cube.face1.point4.z);
            point4 = rotate_x(point4, center, rot_speed);
            let mut point5 = (cube.face4.point1.x, cube.face4.point1.y, cube.face4.point1.z);
            point5 = rotate_x(point5, center, rot_speed);
            let mut point6 = (cube.face4.point2.x, cube.face4.point2.y, cube.face4.point2.z);
            point6 = rotate_x(point6, center, rot_speed);
            let mut point7 = (cube.face4.point3.x, cube.face4.point3.y, cube.face4.point3.z);
            point7 = rotate_x(point7, center, rot_speed);
            let mut point8 = (cube.face4.point4.x, cube.face4.point4.y, cube.face4.point4.z);
            point8 = rotate_x(point8, center, rot_speed);
            cube::new(point{x: point1.0, y: point1.1, z: point1.2}, point{x: point2.0, y: point2.1, z: point2.2}, point{x: point3.0, y: point3.1, z: point3.2}, point{x: point4.0, y: point4.1, z: point4.2}, point{x: point5.0, y: point5.1, z: point5.2}, point{x: point6.0, y: point6.1, z: point6.2}, point{x: point7.0, y: point7.1, z: point7.2}, point{x: point8.0, y: point8.1, z: point8.2})
        }
        
        fn rotate_cube_y(cube: cube, center: &(f32, f32, f32), rot_speed: f32) -> cube{
            let mut point1 = (cube.face1.point1.x, cube.face1.point1.y, cube.face1.point1.z);
            point1 = rotate_y(point1, center, rot_speed);
            let mut point2 = (cube.face1.point2.x, cube.face1.point2.y, cube.face1.point2.z);
            point2 = rotate_y(point2, center, rot_speed);
            let mut point3 = (cube.face1.point3.x, cube.face1.point3.y, cube.face1.point3.z);
            point3 = rotate_y(point3, center, rot_speed);
            let mut point4 = (cube.face1.point4.x, cube.face1.point4.y, cube.face1.point4.z);
            point4 = rotate_y(point4, center, rot_speed);
            let mut point5 = (cube.face4.point1.x, cube.face4.point1.y, cube.face4.point1.z);
            point5 = rotate_y(point5, center, rot_speed);
            let mut point6 = (cube.face4.point2.x, cube.face4.point2.y, cube.face4.point2.z);
            point6 = rotate_y(point6, center, rot_speed);
            let mut point7 = (cube.face4.point3.x, cube.face4.point3.y, cube.face4.point3.z);
            point7 = rotate_y(point7, center, rot_speed);
            let mut point8 = (cube.face4.point4.x, cube.face4.point4.y, cube.face4.point4.z);
            point8 = rotate_y(point8, center, rot_speed);
            cube::new(point{x: point1.0, y: point1.1, z: point1.2}, point{x: point2.0, y: point2.1, z: point2.2}, point{x: point3.0, y: point3.1, z: point3.2}, point{x: point4.0, y: point4.1, z: point4.2}, point{x: point5.0, y: point5.1, z: point5.2}, point{x: point6.0, y: point6.1, z: point6.2}, point{x: point7.0, y: point7.1, z: point7.2}, point{x: point8.0, y: point8.1, z: point8.2})
        }
        
        fn rotate_cube_z(cube: cube, center: &(f32, f32, f32), rot_speed: f32) -> cube{
            let mut point1 = (cube.face1.point1.x, cube.face1.point1.y, cube.face1.point1.z);
            point1 = rotate_z(point1, center, rot_speed);
            let mut point2 = (cube.face1.point2.x, cube.face1.point2.y, cube.face1.point2.z);
            point2 = rotate_z(point2, center, rot_speed);
            let mut point3 = (cube.face1.point3.x, cube.face1.point3.y, cube.face1.point3.z);
            point3 = rotate_z(point3, center, rot_speed);
            let mut point4 = (cube.face1.point4.x, cube.face1.point4.y, cube.face1.point4.z);
            point4 = rotate_z(point4, center, rot_speed);
            let mut point5 = (cube.face4.point1.x, cube.face4.point1.y, cube.face4.point1.z);
            point5 = rotate_z(point5, center, rot_speed);
            let mut point6 = (cube.face4.point2.x, cube.face4.point2.y, cube.face4.point2.z);
            point6 = rotate_z(point6, center, rot_speed);
            let mut point7 = (cube.face4.point3.x, cube.face4.point3.y, cube.face4.point3.z);
            point7 = rotate_z(point7, center, rot_speed);
            let mut point8 = (cube.face4.point4.x, cube.face4.point4.y, cube.face4.point4.z);
            point8 = rotate_z(point8, center, rot_speed);
            cube::new(point{x: point1.0, y: point1.1, z: point1.2}, point{x: point2.0, y: point2.1, z: point2.2}, point{x: point3.0, y: point3.1, z: point3.2}, point{x: point4.0, y: point4.1, z: point4.2}, point{x: point5.0, y: point5.1, z: point5.2}, point{x: point6.0, y: point6.1, z: point6.2}, point{x: point7.0, y: point7.1, z: point7.2}, point{x: point8.0, y: point8.1, z: point8.2})
        }
        
            let mut canvas = Canvas4::new(WIDTH, HEIGHT);
        
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
            // let projection = Mat4x4::perspective(45., WIDTH as f32 / HEIGHT as f32, 0.1, 100.);
            // let projection = Mat4x4::frustum(-1., 1., -1., 1., 1., 100.);
            // let mvp = projection * view * model;
        
            // square is [x-chord, y-chord, z-chor, height, width, x-direction, y-direction, z-direction]
        
            // let mut square = [0.0, 0.3, 0.5, 0.6, 0.6, 0.0, 0.0, 0.0, 0.0];
        
            let camera_pos = [20.0, 20.0, 150.0];
        
            let point = (10.0, 10.0, -10.0);
        
            // let dist_vec = [camera_pos[0] - point[0], camera_pos[1] - point[1], camera_pos[2] - point[2]];
        
            // let proj_point = [point[0]*(abs(point[2])/dist_vec[2]), point[1]*(abs(point[2])/dist_vec[2])];
        
            let point2 = (10.0, 40.0, -10.0);
        
            let point3 = (40.0, 40.0, -10.0);
        
            let point4 = (40.0, 10.0, -10.0);
        
            let point5: (f32, f32, f32) = (10.0, 40.0, -50.0);
        
            let point6 = (10.0, 10.0, -50.0);
        
            let point7: (f32, f32, f32) = (40.0, 10.0, -50.0);
        
            // let point8: (f32, f32, f32) = (10.0, 10.0, -50.0);
        
            // let dist_vec2 = [camera_pos[0] - point2[0], camera_pos[1] - point2[1], camera_pos[2] - point2[2]];
        
            // let proj_point2 = [point2[0]*(abs(point2[2])/dist_vec2[2]), point2[1]*(abs(point2[2])/dist_vec2[2])];
        
            let mut square = [0.0,0.0, 0.5,0.0, 0.5,-0.3, 0.0,-0.3, 0.0,0.0,0.0];
        
            let spin_point = [0.1, 0.2, 0.3];
        
            // let x1_len = square[0] - spin_point[0];
            // let x2_len = square[2] - spin_point[0];
            // let x3_len = square[4] - spin_point[0];
            // let x4_len = square[6] - spin_point[0];
        
            // let y1_len = square[1] - spin_point[1];
            // let y2_len = square[3] - spin_point[1];
            // let y3_len = square[5] - spin_point[1];
            // let y4_len = square[7] - spin_point[1];
        
            let mut Cube = cube::new(point{x: -10.0, y: -10.0, z: -10.0}, point{x: -10.0, y: 10.0, z: -10.0}, point{x: 10.0, y: 10.0, z: -10.0}, point{x: 10.0, y: -10.0, z: -10.0}, point{x: -10.0, y: -10.0, z: 10.0}, point{x: -10.0, y: 10.0, z: 10.0}, point{x: 10.0, y: 10.0, z: 10.0}, point{x: 10.0, y: -10.0, z: 10.0});
        
            let mut face = (point, point2, point3, point4);
            let mut face2 = (point, point2, point5, point6);
            let mut face3 = (point, point4, point7, point6);
        
            let x1_len:f32 = face.0.0 - spin_point[0];
            let x2_len = face.1.0 - spin_point[0];
            let x3_len = face.2.0 - spin_point[0];
            let x4_len = face.3.0 - spin_point[0];
        
            let y1_len:f32 = face.0.1 - spin_point[1];
            let y2_len = face.1.1 - spin_point[1];
            let y3_len = face.2.1 - spin_point[1];
            let y4_len = face.3.1 - spin_point[1];
        
            let z1_len:f32 = face.0.2 - spin_point[2];
            let z2_len:f32 = face.1.2 - spin_point[2];
            let z3_len:f32 = face.2.2 - spin_point[2];
            let z4_len:f32 = face.3.2 - spin_point[2];
        
            // face.4 = x1_len.atan2(z1_len);
            let y_dist = x1_len*x1_len + z1_len*z1_len;
            let y_dist2 = x2_len*x2_len + z2_len*z2_len;
            let y_dist3 = x3_len*x3_len + z3_len*z3_len;
            let y_dist4 = x4_len*x4_len + z4_len*z4_len;
        
        
                let mut rng = rand::thread_rng();
            canvas.set_color(Color::random());
            while window.is_open() && !window.is_key_down(Key::Escape) {
                canvas.clear();
        
                
                // let y_rot: f32 = square[6];
                // let z_rot = square[7];
                let rot_speed: f32 = 0.01;
        
                
                // let x1 = square[0];
                // let y1 = square[1];
                // let x2 = x1 + square[3]*y_rot.cos();
                // let y2 = y1;
                // let x3 = x1 + square[3]*y_rot.cos();
                // let y3 = y1 - square[4]*x_rot.cos();
                // let x4 = x1;
                // let y4 = y1 - square[4]*x_rot.cos();
                // let (x1, y1) = (&mvp * Vec4::from(0., 0.5, 0., 0.)).xy();
                // let (x2, y2) = (&mvp * Vec4::from(-0.5, -0.5, 0., 0.)).xy();
                // let (x3, y3) = (&mvp * Vec4::from(0.5, -0.5, 0., 0.)).xy();
                // canvas.sqr((x1, y1), (x2, y2), (x3, y3), (x4, y4));
                // canvas.sqr((square[0], square[1]), (square[2], square[3]), (square[4], square[5]), (square[6], square[7]));
                // canvas.pixel_3_d(&camera_pos, face.0);
                // canvas.pixel_3_d(&camera_pos, point2);
                // canvas.line_3_d(&camera_pos, face.0, face.1);
                // canvas.sqr_3_d(&camera_pos, face.0, face.1, face.2, face.3);
                // canvas.sqr_3_d(&camera_pos, face2.0, face2.1, face2.2, face2.3);
                // canvas.sqr_3_d(&camera_pos, face3.0, face3.1, face3.2, face3.3);
                // canvas.sqr_3_d(&camera_pos, point2, point3, point8, point5);
                
                // square[5] += rot_speed;
                // square[6] += rot_speed;
                // square[0] = (square[2] * rot_speed.sin()) + (square[0] * rot_speed.cos());
                // square[2] = (square[2] * rot_speed.cos()) + (square[0] * rot_speed.sin());
        
                // square[9] += rot_speed;
                // let x_rot: f32 = square[8];
                // let y_rot: f32 = face.4;
                // face.0.0 = x1_len * x_rot.cos() + spin_point[0];
                // square[2] = x2_len * x_rot.cos() + spin_point[0];
                // square[4] = x3_len * x_rot.cos() + spin_point[0];
                // square[6] = x4_len * x_rot.cos() + spin_point[0];
                // face.0.1 = y1_len * y_rot.cos() + spin_point[1];
                // face.1.1 = y2_len * y_rot.cos() + spin_point[1];
                // face.2.1 = y3_len * y_rot.cos() + spin_point[1];
                // face.3.1 = y4_len * y_rot.cos() + spin_point[1];
        
                show_cube(&Cube, &camera_pos, &mut canvas);
        
                // Cube = rotate_cube_x(Cube, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                Cube = rotate_cube_y(Cube, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                Cube = rotate_cube_z(Cube, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face.0 = rotate_y(face.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.1 = rotate_y(face.1, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.2 = rotate_y(face.2, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.3 = rotate_y(face.3, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face2.0 = rotate_y(face2.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.1 = rotate_y(face2.1, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.2 = rotate_y(face2.2, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.3 = rotate_y(face2.3, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face3.0 = rotate_y(face3.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.1 = rotate_y(face3.1, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.2 = rotate_y(face3.2, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.3 = rotate_y(face3.3, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face.0 = rotate_x(face.0 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.1 = rotate_x(face.1 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.2 = rotate_x(face.2 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.3 = rotate_x(face.3 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face2.0 = rotate_x(face2.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.1 = rotate_x(face2.1, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.2 = rotate_x(face2.2, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.3 = rotate_x(face2.3, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face3.0 = rotate_x(face3.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.1 = rotate_x(face3.1, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.2 = rotate_x(face3.2, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.3 = rotate_x(face3.3, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face2.0 = rotate_z(face2.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.1 = rotate_z(face2.1 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.2 = rotate_z(face2.2 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face2.3 = rotate_z(face2.3 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face.0 = rotate_z(face.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.1 = rotate_z(face.1 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.2 = rotate_z(face.2 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face.3 = rotate_z(face.3 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
                face3.0 = rotate_z(face3.0, &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.1 = rotate_z(face3.1 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.2 = rotate_z(face3.2 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
                face3.3 = rotate_z(face3.3 , &(spin_point[0], spin_point[1], spin_point[2]), rot_speed);
        
        
                // face.0.0 = y_dist as f32 * y_rot.sin() + spin_point[0];
                // face.0.1 = y_dist as f32 * y_rot.cos() + spin_point[1];
        
                // face.1.0 = y_dist2 as f32 * y_rot.sin() + spin_point[0];
                // face.1.1 = y_dist2 as f32 * y_rot.cos() + spin_point[1];
        
                // face.2.0 = y_dist3 as f32 * y_rot.sin() + spin_point[0];
                // face.2.1 = y_dist3 as f32 * y_rot.cos() + spin_point[1];
        
                // face.3.0 = y_dist4 as f32 * y_rot.sin() + spin_point[0];
                // face.3.1 = y_dist4 as f32 * y_rot.cos() + spin_point[1];
        
        
        
                window
                    .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
                    .unwrap();
            }
        }
        
}
