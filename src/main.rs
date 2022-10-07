mod ray;

// crate uses
use crate::ray::Ray;

use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

// for now, this function is the sole decider of the colours in the scene.
fn ray_colour(ray: Ray) -> Rgb<u8> {
    let norm_direction = ray.direction().normalize();
    let t = 0.5 * (norm_direction.y + 1.0);
    let colour_vec = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    Rgb([
        (colour_vec.x * 255.999) as u8,
        (colour_vec.y * 255.999) as u8,
        (colour_vec.z * 255.999) as u8,
    ])
}

fn main() {
    // Image specs
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Rendering

    let img = ImageBuffer::from_fn(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, |x, y| {
        let u = x as f64 / (IMAGE_WIDTH - 1.0);
        let v = y as f64 / (IMAGE_HEIGHT - 1.0);
        let out_ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        ray_colour(out_ray)
    });

    img.save("scene.png");
}
