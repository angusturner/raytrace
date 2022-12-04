use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Option<Arc<dyn Material + Send + Sync>>,
}

impl HitRecord {
    // initialise a record with arbitrary values
    pub fn dummy() -> HitRecord {
        HitRecord {
            p: Point3::zeroes(),
            normal: Vec3::zeroes(),
            t: 0.0,
            front_face: false,
            mat_ptr: None,
        }
    }

    // determine if the ray is inside or outside the object
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.p = other.p;
        self.normal = other.normal;
        self.t = other.t;
        self.front_face = other.front_face;
        self.mat_ptr = match &(other.mat_ptr) {
            Some(val) => Some(Arc::clone(val)),
            None => None,
        };
    }
}
