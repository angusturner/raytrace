use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

// a struct to store all the "Hittable" objects in our scene.
pub struct HittableList {
    components: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.components.push(obj);
    }

    pub fn clear(&mut self) {
        self.components.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::dummy();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for item in self.components.iter() {
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.copy_from(&temp_rec);
            }
        }

        return hit_anything;
    }
}
