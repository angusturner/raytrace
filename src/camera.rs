use crate::random::random_in_unit_disk;
use crate::ray::Ray;
use crate::util::degrees_to_radians;
use crate::vec3::{Point3, Vec3};
use rand::rngs::ThreadRng;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f64,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Point3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
            lens_radius,
            u,
            v,
        }
    }
}

impl Camera {
    // get a ray from the camera to the position (s, t) on the viewport, where `s` and `t`
    // are normalised co-ordinates in [0, 1]
    pub fn get_ray(&self, s: f64, t: f64, gen: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(gen);
        let offset = self.u * rd.x + self.v * rd.y;

        let dir: Vec3 =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        Ray {
            origin: self.origin + offset,
            dir,
        }
    }
}
