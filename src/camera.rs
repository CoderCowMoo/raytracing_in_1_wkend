use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Camera {
    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self {
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            horizontal: Vector3::new(viewport_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, viewport_height, 0.0),
            lower_left_corner: Vector3::new(0.0, 0.0, 0.0)
                - Vector3::new(viewport_width, 0.0, 0.0) / 2.0
                - Vector3::new(0.0, viewport_height, 0.0) / 2.0
                - Vector3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
