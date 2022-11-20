use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // initialise a record with arbitrary values
    pub fn dummy() -> HitRecord {
        HitRecord {
            p: Point3::zeroes(),
            normal: Vec3::zeroes(),
            t: 0.0,
            front_face: false,
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
    }
}
