use crate::ppm::render_ppm;
use crate::vec3::Vec3;

mod ppm;
mod vec3;

fn main() {
    let x = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 0.0,
    };
    let y = Vec3 {
        x: 2.0,
        y: 2.0,
        z: 0.0,
    };
    println!("{}, {}, {}", x.length(), y.length(), (x + y).length());
    // render_ppm()
}
