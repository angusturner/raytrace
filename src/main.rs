use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::util::write_color;
use crate::vec3::{Color, Point3};
use std::rc::Rc;

use crate::build_random_scene::build_random_scene;
use crate::material::Material;
use rand::rngs::ThreadRng;
use rand::Rng;

mod build_random_scene;
mod camera;
mod dielectric;
mod hit_record;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod random;
mod ray;
mod sphere;
mod util;
mod vec3;

const SAMPLES_PER_PIXEL: u32 = 500;
const IMAGE_WIDTH: u32 = 800;
const MAX_DEPTH: u32 = 50;

fn ray_color(ray: &Ray, world: &HittableList, depth: u32, gen: &mut ThreadRng) -> Color {
    let mut record = HitRecord::dummy();

    // no more light gathered if depth exceeded
    if depth == 0 {
        return Color::zeroes();
    }

    // check for an intersection
    if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        // get the material pointer from the hit-record
        let mat_ptr: Rc<dyn Material> = match &record.mat_ptr {
            Some(val) => Rc::clone(val),
            None => panic!(),
        };

        // check for scattered ray on that material
        return match mat_ptr.scatter(&ray, &record, gen) {
            Some((reflected_ray, attenuation)) => {
                attenuation * ray_color(&reflected_ray, &world, depth - 1, gen)
            }
            None => Color::zeroes(),
        };
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
    let aspect_ratio: f64 = 3.0 / 2.0;
    let aperture = 0.1;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let image_height: u32 = ((IMAGE_WIDTH as f64) / aspect_ratio) as u32;

    // world
    let world = build_random_scene();

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
                let ray = camera.get_ray(u, v, &mut gen);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH, &mut gen);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
