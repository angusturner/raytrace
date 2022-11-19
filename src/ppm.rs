use crate::util::write_color;
use crate::vec3::Color;
use std::io::stdout;

pub fn render_ppm() {
    const HEIGHT: u32 = 256;
    const WIDTH: u32 = 256;
    println!("P3 {} {} 255", HEIGHT, WIDTH);
    for j in (0..HEIGHT).rev() {
        eprint!("\rScan lines remaining: {}", j);
        for i in 0..WIDTH {
            let color = Color {
                x: (i as f64) / (WIDTH as f64 - 1.0),
                y: (j as f64) / (HEIGHT as f64 - 1.0),
                z: 0.25,
            };
            write_color(color);
        }
    }
}
