use crate::dielectric::Dielectric;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;
use std::rc::Rc;

type RcMaterial = Rc<dyn Material>;

pub fn build_random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mut gen = rand::thread_rng();

    // ground
    let ground_mat: RcMaterial = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    let ground = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: ground_mat,
    };
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = gen.gen::<f64>();
            let r1 = gen.gen::<f64>();
            let r2 = gen.gen::<f64>();
            let center = Point3 {
                x: a as f64 + 0.9 * r1,
                y: 0.2,
                z: b as f64 + 0.9 * r2,
            };

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: RcMaterial;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut gen) * Color::random(&mut gen);
                    sphere_mat = Rc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(&mut gen) * 0.5 + 0.5; // [0.5, 1)
                    let fuzz = gen.gen::<f64>() * 0.5;
                    sphere_mat = Rc::new(Metal { albedo, fuzz });
                } else {
                    // glass
                    sphere_mat = Rc::new(Dielectric { ir: 1.5 });
                }

                let sphere = Sphere {
                    center,
                    radius: 0.2,
                    mat_ptr: sphere_mat,
                };
                world.add(Box::new(sphere));
            }
        }
    }

    let mat1: RcMaterial = Rc::new(Dielectric { ir: 1.5 });
    let sphere1 = Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: mat1,
    };
    world.add(Box::new(sphere1));

    let mat2: RcMaterial = Rc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let sphere2 = Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: mat2,
    };
    world.add(Box::new(sphere2));

    let mat3: RcMaterial = Rc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    let sphere3 = Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: mat3,
    };
    world.add(Box::new(sphere3));

    return world;
}
