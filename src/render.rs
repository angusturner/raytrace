use crate::environment::Environment;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::image::Image;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Color;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::sync::Arc;

pub fn render(
    env: Arc<Environment>,
    samples_per_pixel: u32,
    image_width: u32,
    max_depth: u32,
) -> Image {
    let image_height = ((image_width as f64) / env.camera.aspect_ratio) as u32;

    let mut gen = rand::thread_rng();

    // println!("P3 {} {} 255", image_width, image_height);
    let mut pixels: Vec<Vec<Color>> = Vec::new();
    for j in (0..image_height).rev() {
        let mut row: Vec<Color> = Vec::new();
        for i in 0..image_width {
            let mut pixel_color = Color::zeroes();
            for _ in 0..samples_per_pixel {
                let noise_u = gen.gen::<f64>();
                let noise_v = gen.gen::<f64>();
                let u: f64 = (i as f64 + noise_u) / (image_width as f64 - 1.0); // 0.0 to 1.0
                let v: f64 = (j as f64 + noise_v) / (image_height as f64 - 1.0);
                let ray = env.camera.get_ray(u, v, &mut gen);
                pixel_color += ray_color(&ray, &env.world, max_depth, &mut gen);
            }
            row.push(pixel_color / (samples_per_pixel as f64));
        }
        pixels.push(row);
    }

    return Image {
        width: image_width,
        height: image_height,
        pixels,
    };
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32, gen: &mut ThreadRng) -> Color {
    let mut record = HitRecord::dummy();

    // no more light gathered if depth exceeded
    if depth == 0 {
        return Color::zeroes();
    }

    // check for an intersection
    if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        // get the material pointer from the hit-record
        let mat_ptr: Arc<dyn Material + Send + Sync> = match &record.mat_ptr {
            Some(val) => Arc::clone(val),
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
