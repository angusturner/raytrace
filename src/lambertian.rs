use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::random::random_on_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Color;
use rand::rngs::ThreadRng;

// a diffuse material which randomly reflect rays
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord, gen: &mut ThreadRng) -> Option<(Ray, Color)> {
        let mut scatter_dir = record.normal + random_on_unit_sphere(gen);
        if scatter_dir.near_zero() {
            scatter_dir = record.normal;
        }
        let scattered = Ray {
            origin: record.p,
            dir: scatter_dir,
        };
        Some((scattered, self.albedo))
    }
}
