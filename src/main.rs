mod buffer;
mod canvas;
mod color;
mod linalg;

use std::{thread, time};
use canvas::Canvas;
use color::Color;
use linalg::matrix::Mat4x4;
use linalg::vec3::Vec3;
use std::f32::consts::PI;

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
            *vertex = rotation_matrix.transform_vector(*vertex);
        }
    }
    
}

fn draw_cube(canvas: &mut Canvas, cube: &Cube, color: Color) {
    for i in 0..cube.vertices.len() {
        let start = cube.vertices[i];
        let end = cube.vertices[(i + 1) % cube.vertices.len()];
        let (sx, sy) = project_to_screen(start);
        let (ex, ey) = project_to_screen(end);
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


fn main() {
    let mut window = minifb::Window::new(
        "Test - ESC to exit",
        800,
        600,
        minifb::WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    let mut canvas = Canvas::new(800, 600);
    let mut cube = Cube::new(); 

    let mut angle = 0.0;
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let axis = Vec3::new(0.0, 1.0, 0.0);  
        angle += PI / 180.0;  
        cube.rotate(angle, axis);

        canvas.clear();
        draw_cube(&mut canvas, &cube, Color::BLUE);
        thread::sleep(time::Duration::from_millis(1));
        draw_cube(&mut canvas, &cube, Color::GREEN);
        thread::sleep(time::Duration::from_millis(3));
        draw_cube(&mut canvas, &cube, Color::BLACK);
        thread::sleep(time::Duration::from_millis(4));
        draw_cube(&mut canvas, &cube, Color::RED);
  


        window.update_with_buffer(canvas.buffer(), canvas.width(), canvas.height()).unwrap();
    }
}
