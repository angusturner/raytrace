use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::write_color;
use crate::vec3::{Color, Point3};

use rand::Rng;

mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

const SAMPLES_PER_PIXEL: u32 = 100;
const IMAGE_WIDTH: u32 = 1280;

fn ray_color(ray: &Ray, world: &mut HittableList) -> Color {
    // check for an intersection, and if one is found shade according to the sphere surface normal
    let mut record = HitRecord::dummy();
    let is_hit = world.hit(&ray, 0.0, 100.0, &mut record);
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
    // image + camera
    let camera = Camera::new();
    let aspect_ratio: f64 = camera.aspect_ratio;
    let image_height: u32 = ((IMAGE_WIDTH as f64) / aspect_ratio) as u32;

    // world
    let mut world = HittableList::new();
    let sphere1 = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    world.add(Box::new(sphere1));
    let sphere2 = Sphere {
        center: Point3::new(1.0, 0.0, -2.0),
        radius: 0.5,
    };
    world.add(Box::new(sphere2));

    // rng generator
    let mut gen = rand::thread_rng();

    // render!
    println!("P3 {} {} 255", IMAGE_WIDTH, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScan lines remaining: {:0>4}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zeroes();
            for _ in 0..SAMPLES_PER_PIXEL {
                let noise_u = gen.gen::<f64>();
                let noise_v = gen.gen::<f64>();
                let u: f64 = (i as f64 + noise_u) / (IMAGE_WIDTH as f64 - 1.0); // 0.0 to 1.0
                let v: f64 = (j as f64 + noise_v) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &mut world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
