use crate::camera::Camera;
use crate::hittable_list::HittableList;

pub struct Environment {
    pub camera: Camera,
    pub world: HittableList,
}
