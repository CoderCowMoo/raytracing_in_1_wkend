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

fn ray_colour(colour_non_mapped: Vector3<f64>, samples_per_pixel: u32) -> Rgb<u8> {
    let r = (colour_non_mapped.x).sqrt();
    let g = (colour_non_mapped.y).sqrt();
    let b = (colour_non_mapped.z).sqrt();
    Rgb([
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    ])
}

// returns a non mapped (0.0 .. 1.0) Vector3 of R G B
fn ray_colour_non_manip(ray: &Ray, world: &HittableList, depth: u32) -> Vector3<f64> {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(*ray, 0.001, f64::MAX) {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        0.5 * ray_colour_non_manip(&Ray::new(hit.point, target - hit.point), &world, depth - 1)
    } else {
        // background (skybox now)
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

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

fn main() {
    // cache rand function
    let mut rng = rand::thread_rng();
    // Image specs
    const ASPECT_RATIO: f64 = 2.0; // instead of 16.0 / 9.0
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let camera = Camera::new(viewport_width, viewport_height, focal_length);

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
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH;
                let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT;
                let ray = camera.get_ray(u, v);
                colour += ray_colour_non_manip(&ray, &world, MAX_DEPTH);
            }
            colour /= SAMPLES_PER_PIXEL as f64;
            let out_colour = ray_colour(colour, SAMPLES_PER_PIXEL);
            img.put_pixel(i, IMAGE_HEIGHT as u32 - j, out_colour);
        }
    }

    img.save("scene.png").unwrap();
}
