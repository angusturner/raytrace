use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::write_color;
use crate::vec3::{Color, Point3, Vec3};

mod hittable;
mod ray;
mod sphere;
mod util;
mod vec3;

fn ray_color(ray: &Ray) -> Color {
    // add a single sphere to the scene
    let sphere = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    // check for an intersection, and if one is found shade according to the sphere surface normal
    let mut record = HitRecord::dummy();
    let is_hit = sphere.hit(&ray, 0.0, 100.0, &mut record);
    if is_hit {
        return 0.5 * (record.normal + 1.0);
    }

    // otherwise, shade background according to the y-component of the normalised ray direction
    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
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
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // render!
    println!("P3 {} {} 255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScan lines remaining: {}", j);
        for i in 0..image_width {
            // the vectors u and v define the horizontal and vertical position of pixel on the
            // viewport, relative to the lower left corner
            let u_frac: f64 = (i as f64) / (image_width as f64 - 1.0); // 0.0 to 1.0
            let v_frac: f64 = (j as f64) / (image_height as f64 - 1.0);
            let u: Vec3 = u_frac * horizontal;
            let v: Vec3 = v_frac * vertical;

            // get the ray direction from the origin through the pixel
            let dir: Vec3 = lower_left_corner + u + v - origin;
            let ray = Ray { origin, dir };
            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
}
