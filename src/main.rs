use crate::camera::Camera;
use crate::vec3::Point3;
use std::sync::Arc;

use crate::build_random_scene::build_random_scene;
use crate::environment::Environment;
use crate::image::Image;
use crate::render::render;
use crate::util::write_color;

mod build_random_scene;
mod camera;
mod dielectric;
mod environment;
mod hit_record;
mod hittable;
mod hittable_list;
mod image;
mod lambertian;
mod material;
mod metal;
mod random;
mod ray;
mod render;
mod sphere;
mod util;
mod vec3;

const SAMPLES_PER_PIXEL: u32 = 16;
const IMAGE_WIDTH: u32 = 900;
const MAX_DEPTH: u32 = 100;
const NUM_THREADS: u32 = 8;

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
    let env = Environment { camera, world };

    // wrap the environment in an Arc, so that it can be passed between threads.
    let env = Arc::new(env);

    // spawn `NUM_THREADS` threads, each of which will render an image. average the resulting images
    // to get the final image.
    let mut threads = Vec::new();
    for _ in 0..NUM_THREADS {
        let env = Arc::clone(&env);
        threads.push(std::thread::spawn(move || {
            let image = render(env, SAMPLES_PER_PIXEL, IMAGE_WIDTH, MAX_DEPTH);
            return image;
        }));
    }

    // create an image of all zeroes
    let mut image = Image::zeroes(IMAGE_WIDTH, image_height);

    // collect the images from the threads
    for thread in threads {
        let image_i = thread.join().unwrap();
        image += image_i;
    }

    // render!
    println!("P3 {} {} 255", IMAGE_WIDTH, image_height);
    for row in image.pixels {
        for pixel in row {
            write_color(pixel, NUM_THREADS, 0.5);
        }
    }
    eprintln!("Writing image...");
}
