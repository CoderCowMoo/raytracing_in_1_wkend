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

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal/fuzzyreflection:~:text=(typically%20air%20%3D%201.0%2C%20glass%20%3D%201.3%E2%80%931.7%2C%20diamond%20%3D%202.4)
fn refract(v: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - etai_over_etat.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = etai_over_etat * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub struct Dielectric {
    ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt) = if ray.direction().dot(&rec.normal) > 0.0 {
            (-rec.normal, self.ir)
        } else {
            (rec.normal, 1.0 / self.ir)
        };
        if let Some(refracted) = refract(ray.direction(), outward_normal, ni_over_nt) {
            let scattered = Ray::new(rec.point, refracted);
            Some((scattered, attenuation))
        } else {
            let reflected = reflect(&ray.direction(), &rec.normal);
            let scattered = Ray::new(rec.point, reflected);
            Some((scattered, attenuation))
        }
    }
}
