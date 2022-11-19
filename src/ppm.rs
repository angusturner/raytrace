use std::io::{stdout};

pub fn render_ppm() {
    const HEIGHT: u32 = 256;
    const WIDTH: u32 = 256;
    println!("P3 {} {} 255", HEIGHT, WIDTH);
    for j in (0..HEIGHT).rev() {
        eprint!("\rScan lines remaining: {}", j);
        for i in 0..WIDTH {
            let r: f32 = (i as f32) / (WIDTH as f32 - 1.0);
            let g: f32 = (j as f32) / (HEIGHT as f32 - 1.0);
            let b: f32 = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
