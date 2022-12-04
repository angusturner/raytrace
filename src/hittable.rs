use crate::hit_record::HitRecord;
use crate::ray::Ray;

// trait for all hittable surfaces
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
