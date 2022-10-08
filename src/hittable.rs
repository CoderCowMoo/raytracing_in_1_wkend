use nalgebra::Vector3;

use crate::ray::Ray;

pub struct HitRecord {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub point: Vector3<f64>,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max; // set closest so far to be the furthest in valid range
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        } // we'll either hit something in the for loop or never hit something.
        hit_anything
    }
}
