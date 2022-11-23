use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::write_color;
use crate::vec3::{Color, Point3};
use std::rc::Rc;

use crate::dielectric::Dielectric;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use rand::rngs::ThreadRng;
use rand::Rng;

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

const SAMPLES_PER_PIXEL: u32 = 100;
const IMAGE_WIDTH: u32 = 640;
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

type RcMaterial = Rc<dyn Material>;

fn main() {
    // image + camera
    let camera = Camera::new();
    let aspect_ratio: f64 = camera.aspect_ratio;
    let image_height: u32 = ((IMAGE_WIDTH as f64) / aspect_ratio) as u32;

    // world
    let mut world = HittableList::new();

    let glass: RcMaterial = Rc::new(Dielectric { ir: 1.5 });
    let diffuse_cream: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(252.0 / 255.0, 246.0 / 255.0, 177.0 / 255.0),
    });
    let diffuse_magenta: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(124.0 / 255.0, 36.0 / 255.0, 148.0 / 255.0),
    });
    let diffuse_pink: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(252.0 / 255.0, 109.0 / 255.0, 171.0 / 255.0),
    });
    let diffuse_green: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(169.0 / 255.0, 229.0 / 255.0, 187.0 / 255.0),
    });
    let diffuse_orange: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(247.0 / 255.0, 179.0 / 255.0, 43.0 / 255.0),
    });
    let metallic_blue: RcMaterial = Rc::new(Metal {
        albedo: Color::new(183.0 / 255.0, 192.0 / 255.0, 238.0 / 255.0),
        fuzz: 0.3,
    });
    let metallic_grey: RcMaterial = Rc::new(Metal {
        albedo: Color::new(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0),
        fuzz: 0.05,
    });

    // vertical spheres
    let sphere_center = Sphere {
        center: Point3::new(0.0, -0.5, -1.25),
        radius: 0.25,
        mat_ptr: diffuse_magenta,
    };
    world.add(Box::new(sphere_center));
    let sphere_center_b = Sphere {
        center: Point3::new(0.0, 0.0, -1.25),
        radius: 0.25,
        mat_ptr: Rc::clone(&glass),
    };
    world.add(Box::new(sphere_center_b));
    let sphere_center_c = Sphere {
        center: Point3::new(0.0, 0.5, -1.25),
        radius: 0.25,
        mat_ptr: diffuse_green,
    };
    world.add(Box::new(sphere_center_c));

    // horizontal spheres
    let sphere_center_back = Sphere {
        center: Point3::new(-2.0, 0.0, -2.25),
        radius: 0.25,
        mat_ptr: diffuse_pink,
    };
    world.add(Box::new(sphere_center_back));
    let sphere_center_b_back = Sphere {
        center: Point3::new(0.0, 0.0, -2.25),
        radius: 0.25,
        mat_ptr: metallic_blue,
    };
    world.add(Box::new(sphere_center_b_back));
    let sphere_center_c_back = Sphere {
        center: Point3::new(2.0, 0.0, -2.25),
        radius: 0.25,
        mat_ptr: diffuse_orange,
    };
    world.add(Box::new(sphere_center_c_back));

    // ground + sky sphere
    let sphere_ground = Sphere {
        center: Point3::new(0.0, -100.75, -1.25),
        radius: 100.0,
        mat_ptr: diffuse_cream,
    };
    world.add(Box::new(sphere_ground));
    let sphere_sky = Sphere {
        center: Point3::new(0.0, 20.75, -1.25),
        radius: 20.0,
        mat_ptr: metallic_grey,
    };
    world.add(Box::new(sphere_sky));

    // hollow-sphere on left
    let sphere_left = Sphere {
        center: Point3::new(-1.0, -0.0, -1.25),
        radius: 0.75,
        mat_ptr: Rc::clone(&glass), // mat_ptr: material_left,
    };
    world.add(Box::new(sphere_left));
    let sphere_left_b = Sphere {
        center: Point3::new(-1.0, -0.0, -1.25),
        radius: -0.60,
        mat_ptr: Rc::clone(&glass), // mat_ptr: material_left,
    };
    world.add(Box::new(sphere_left_b));

    // hollow-sphere on right
    let sphere_right = Sphere {
        center: Point3::new(1.0, -0.0, -1.25),
        radius: 0.75,
        mat_ptr: Rc::clone(&glass), // mat_ptr: material_left,
    };
    world.add(Box::new(sphere_right));
    let sphere_right_b = Sphere {
        center: Point3::new(1.0, -0.0, -1.25),
        radius: -0.60,
        mat_ptr: Rc::clone(&glass), // mat_ptr: material_left,
    };
    world.add(Box::new(sphere_right_b));

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
                pixel_color += ray_color(&ray, &world, MAX_DEPTH, &mut gen);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
