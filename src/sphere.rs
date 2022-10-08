use nalgebra::Vector3;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Sphere<M: Material> {
    pos: Vector3<f64>,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(position: Vector3<f64>, radius: f64, material: M) -> Self {
        Sphere {
            pos: position,
            radius: radius,
            material,
        }
    }
    pub fn pos(&self) -> Vector3<f64> {
        self.pos
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.pos();
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius().powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let point = ray.point_when_at(t);
                let normal = (point - self.pos()) / self.radius;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material: &self.material,
                });
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let point = ray.point_when_at(t);
                let normal = (point - self.pos()) / self.radius;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material: &self.material,
                });
            }
        }
        None
    }
}
