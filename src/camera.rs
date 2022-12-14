use crate::ray::Ray;

use nalgebra::Vector3;
use rand::Rng;
use std::f64;

fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - unit;
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        view_up: Vector3<f64>,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.0;
        let height = (theta / 2.0).tan() * focus_dist * 2.0;
        let width = aspect * height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - (width / 2.0) * u - (height / 2.0) * v - focus_dist * w,
            horizontal: width * u,
            vertical: height * v,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
