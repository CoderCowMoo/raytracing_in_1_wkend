mod objects;
mod ray;

// crate uses
use crate::objects::Sphere;
use crate::ray::Ray;

use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

// we are trying to calculate the value t for Rays in finding points on the Ray where it intersects with the sphere
fn sphere_intersect(center_of_sphere: Vector3<f64>, radius: f64, ray: Ray) -> Option<f64> {
    // I'm not sure exactly oc is but I'll find out eventually
    let oc = ray.origin() - center_of_sphere;
    // now we have a quadratic equation which we're getting the coefficients for:
    // ax^2 + bx + c
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    } else {
        return Some((-b - discriminant.sqrt()) / (2.0 * a));
    }
}

// for now, this function is the sole decider of the colours in the scene.
fn ray_colour(ray: Ray) -> Rgb<u8> {
    // here we define sphere of size 0.5 and at position -1 from camera in mid.
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(t) = sphere_intersect(sphere.pos(), sphere.radius(), ray) {
        let n = (ray.point_when_at(t) - sphere.pos()).normalize();
        let colour_vec = 0.5 * n.add_scalar(1.0);
        Rgb([
            (colour_vec.x * 255.99) as u8,
            (colour_vec.y * 255.99) as u8,
            (colour_vec.z * 255.99) as u8,
        ])
    } else {
        let norm_direction = ray.direction().normalize();
        let t = 0.5 * (norm_direction.y + 1.0);
        // lerp the 2 colours sky blue and white
        let colour_vec =
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + (t) * Vector3::new(0.5, 0.7, 1.0);
        Rgb([
            (colour_vec.x * 255.99) as u8,
            (colour_vec.y * 255.99) as u8,
            (colour_vec.z * 255.99) as u8,
        ])
    }
}

fn main() {
    // Image specs
    const ASPECT_RATIO: f64 = 2.0;
    const IMAGE_WIDTH: f64 = 200.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);

    // Rendering

    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for j in (1..IMAGE_HEIGHT as u32).rev() {
        for i in 0..IMAGE_WIDTH as u32 {
            let u = i as f64 / (IMAGE_WIDTH);
            let v = j as f64 / (IMAGE_HEIGHT);
            let new_direction = lower_left_corner + u * horizontal + v * vertical;
            let out_ray = Ray::new(origin, new_direction);
            let out_colr = ray_colour(out_ray);
            // println!("{} {} {}", out_colr.0[0], out_colr.0[1], out_colr[2]);
            img.put_pixel(i, IMAGE_HEIGHT as u32 - j, out_colr);
        }
    }

    img.save("scene.png").unwrap();
}
