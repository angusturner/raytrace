use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.dir.length_squared(); // equivalent to `dir.dot(dir)`
        let half_b = ray.dir.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        // find closest intersecting point that satisfies t_min < t < t_max
        let out_of_bounds = |t| t < t_min || t_max < t;
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if out_of_bounds(root) {
            root = (-half_b + sqrtd) / a;
            if out_of_bounds(root) {
                return false;
            }
        }

        // update the hit-record
        record.t = root;
        record.p = ray.at(root);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(&ray, outward_normal);
        return true;
    }
}
