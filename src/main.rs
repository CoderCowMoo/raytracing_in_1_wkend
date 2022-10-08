mod hittable;
mod ray;
mod sphere;

// crate uses
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;

use hittable::Hittable;
use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

// for now, this function is the sole decider of the colours in the scene.
fn ray_colour(ray: &Ray, world: &HittableList) -> Rgb<u8> {
    if let Some(hit) = world.hit(*ray, 0.0, f64::MAX) {
        let out_col = 0.5 * hit.normal.add_scalar(1.0);
        Rgb([
            (out_col.x * 255.999) as u8,
            (out_col.y * 255.999) as u8,
            (out_col.z * 255.999) as u8,
        ])
    } else {
        // background (skybox now)
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let out_col = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        Rgb([
            (out_col.x * 255.999) as u8,
            (out_col.y * 255.999) as u8,
            (out_col.z * 255.999) as u8,
        ])
    }
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

    // define world and scene
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Rendering

    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for j in (1..IMAGE_HEIGHT as u32).rev() {
        for i in 0..IMAGE_WIDTH as u32 {
            let u = i as f64 / (IMAGE_WIDTH);
            let v = j as f64 / (IMAGE_HEIGHT);
            let new_direction = lower_left_corner + u * horizontal + v * vertical;
            let out_ray = Ray::new(origin, new_direction);
            let out_colr = ray_colour(&out_ray, &world);
            // println!("{} {} {}", out_colr.0[0], out_colr.0[1], out_colr[2]);
            img.put_pixel(i, IMAGE_HEIGHT as u32 - j, out_colr);
        }
    }

    img.save("scene.png").unwrap();
}
