use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::metal::reflect;
use crate::random::random_double;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f64;

fn refract(unit_direction: Vec3, normal: Vec3, eta_on_eta_prime: f64, cos_theta: f64) -> Vec3 {
    let r_out_orth = eta_on_eta_prime * (unit_direction + cos_theta * normal);
    let r_out_par = -f64::sqrt(f64::abs(1.0 - r_out_orth.length_squared())) * normal;
    return r_out_orth + r_out_par;
}

pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Material for Dielectric {
    // todo: convince yourself this derivation is correct...
    fn scatter(&self, ray: &Ray, record: &HitRecord, gen: &mut ThreadRng) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match record.front_face {
            true => 1.0 / self.ir, // note: assumes `n` = 1.0 for air
            false => self.ir,
        };
        let unit_dir = ray.dir.unit_vector();
        let cos_theta = f64::min((-unit_dir).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        // determine whether to refract or reflect
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let dir: Vec3;
        if cannot_refract || (Self::reflectance(cos_theta, refraction_ratio) > gen.gen::<f64>()) {
            dir = reflect(unit_dir, record.normal);
        } else {
            dir = refract(unit_dir, record.normal, refraction_ratio, cos_theta);
        }

        let scattered = Ray {
            origin: record.p,
            dir,
        };

        return Some((scattered, attenuation));
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        // Schlick's approximation
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
