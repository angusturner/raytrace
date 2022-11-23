use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::rngs::ThreadRng;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - (2.0 * v.dot(&n) * n);
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord, gen: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflected = reflect(ray.dir.unit_vector(), record.normal);
        let scattered = Ray {
            origin: record.p,
            dir: reflected + self.fuzz * random_in_unit_sphere(gen),
        };
        if scattered.dir.dot(&record.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }
        return None;
    }
}
