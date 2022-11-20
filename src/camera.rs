use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zeroes();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
        }
    }
}

impl Camera {
    // get a ray from the camera to the position (u, v) on the viewport, where `u` and `v`
    // are normalised co-ordinates in [0, 1]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir: Vec3 =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray {
            origin: self.origin,
            dir,
        }
    }
}
