use crate::vec3::Vec3;
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn random_double(min: Option<f64>, max: Option<f64>, gen: &mut ThreadRng) -> f64 {
    let u = gen.gen::<f64>();
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1.0);
    min + (max - min) * u
}

// sample a random vector from the surface of the unit sphere, bounded in [-1, 1]
pub fn random_on_unit_sphere(gen: &mut ThreadRng) -> Vec3 {
    let mut u = [0f64; 3];
    loop {
        gen.fill(&mut u);
        let vec = Vec3::new(u[0], u[1], u[2]);
        if vec.length_squared() >= 1.0 {
            continue;
        }
        return vec.unit_vector();
    }
}
