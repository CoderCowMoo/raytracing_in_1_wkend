use nalgebra::Vector3;
use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;

fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit_vec = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let point =
            2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit_vec;
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.point, target - rec.point);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(&ray.direction().normalize(), &rec.normal);
        let reflected_fuzzed = reflected + self.fuzz * random_in_unit_sphere();
        if reflected_fuzzed.dot(&rec.normal) > 0.0 {
            let scattered = Ray::new(rec.point, reflected_fuzzed);
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
