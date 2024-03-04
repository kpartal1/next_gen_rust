mod buffer;
mod color;
mod linalg;

use buffer::Buffer;
use color::Color;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);

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

    let mut rng = rand::thread_rng();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear();
        buffer.set_color(Color::random());
        let x1 = rng.gen_range(0..WIDTH);
        let y1 = rng.gen_range(0..HEIGHT);
        let x2 = rng.gen_range(0..WIDTH);
        let y2 = rng.gen_range(0..HEIGHT);
        let x3 = rng.gen_range(0..WIDTH);
        let y3 = rng.gen_range(0..HEIGHT);
        buffer.tri((x1, y1), (x2, y2), (x3, y3));

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
