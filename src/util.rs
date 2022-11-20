use crate::vec3::Color;

pub fn write_color(color: Color) {
    let ir: u32 = (255.999 * color.x) as u32;
    let ig: u32 = (255.999 * color.y) as u32;
    let ib: u32 = (255.999 * color.z) as u32;
    println!("{} {} {}", ir, ig, ib);
}

pub fn random_double(min: Option<f64>, max: Option<f64>) -> f64 {
    let u = rand::random::<f64>();
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1.0);
    min + (max - min) * u
}
