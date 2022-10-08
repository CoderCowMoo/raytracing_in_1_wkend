mod camera;
mod hittable;
mod ray;
mod sphere;

// crate uses
use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;

use hittable::Hittable;
use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;
use rand::Rng;

// returns a non mapped (0.0 .. 1.0) Vector3 of R G B
fn ray_colour(ray: &Ray, world: &HittableList) -> Vector3<f64> {
    if let Some(hit) = world.hit(*ray, 0.0, f64::MAX) {
        0.5 * hit.normal.add_scalar(1.0)
    } else {
        // background (skybox now)
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // cache rand function
    let mut rng = rand::thread_rng();
    // Image specs
    const ASPECT_RATIO: f64 = 2.0; // instead of 16.0 / 9.0
    const IMAGE_WIDTH: f64 = 200.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const SAMPLES_PER_PIXEL: f64 = 100.0;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let camera = Camera::new(viewport_width, viewport_height, focal_length);

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);

    // define world and scene
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Rendering

    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for j in (1..IMAGE_HEIGHT as u32).rev() {
        for i in 0..IMAGE_WIDTH as u32 {
            let mut colour = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL as u32 {
                let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH;
                let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT;
                let ray = camera.get_ray(u, v);
                colour += ray_colour(&ray, &world);
            }
            colour /= SAMPLES_PER_PIXEL;
            let out_colour = Rgb([
                (colour.x * 255.999) as u8,
                (colour.y * 255.999) as u8,
                (colour.z * 255.999) as u8,
            ]);
            img.put_pixel(i, IMAGE_HEIGHT as u32 - j, out_colour);
        }
    }

    img.save("scene.png").unwrap();
}
