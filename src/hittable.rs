use nalgebra::Vector3;

use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub t: f64,
    pub normal: Vector3<f64>,
    pub point: Vector3<f64>,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// default returns default for a type (in this case what new would return)
#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable));
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
