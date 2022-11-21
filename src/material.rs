use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use rand::rngs::ThreadRng;

pub trait Material {
    // take a ray and a hit-record. we can absorb, OR return a scattered ray with an attenuation
    fn scatter(&self, ray: &Ray, record: &HitRecord, gen: &mut ThreadRng) -> Option<(Ray, Color)>;
}
