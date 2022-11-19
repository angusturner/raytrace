use crate::ppm::render_ppm;
use crate::ray::Ray;
use crate::util::write_color;
use crate::vec3::{Color, Point3, Vec3};

mod ppm;
mod ray;
mod util;
mod vec3;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let blue = Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    (1.0 - t) * white + t * blue
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zeroes();
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // render!
    println!("P3 {} {} 255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScan lines remaining: {}", j);
        for i in 0..image_width {
            let u: f64 = (i as f64) / (image_width as f64 - 1.0);
            let v: f64 = (j as f64) / (image_height as f64 - 1.0);
            let dir: Vec3 = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray { origin, dir };
            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
}
