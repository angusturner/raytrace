use crate::vec3::Color;
use std::f64::consts::PI;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    2.0 * PI * degrees / 360.0
}

pub fn write_color(color: Color, samples_per_pixel: u32, gamma_correction: f64) {
    let color_scaled = (color / (samples_per_pixel as f64)).powf(gamma_correction);
    let ir: u32 = (256.0 * clamp(color_scaled.x, 0.0, 0.999)) as u32;
    let ig: u32 = (256.0 * clamp(color_scaled.y, 0.0, 0.999)) as u32;
    let ib: u32 = (256.0 * clamp(color_scaled.z, 0.0, 0.999)) as u32;
    println!("{} {} {}", ir, ig, ib);
}
