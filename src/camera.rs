use crate::ray::Ray;

use nalgebra::Vector3;
use std::f64;

pub struct Camera {
    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        view_up: Vector3<f64>,
        vertical_fov: f64,
        aspect: f64,
    ) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.0;
        let height = (theta / 2.0).tan() * 2.0;
        let width = aspect * height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - (width / 2.0) * u - (height / 2.0) * v - w,
            horizontal: width * u,
            vertical: height * v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
