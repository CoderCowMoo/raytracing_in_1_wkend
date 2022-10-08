mod ray;

// crate uses
use crate::ray::Ray;

use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

// we are trying to calculate the value t for Rays in finding points on the Ray where it intersects with the sphere
fn sphere_intersect(center_of_sphere: Vector3<f64>, radius: f64, ray: Ray) -> bool {
    // I'm not sure exactly oc is but I'll find out eventually
    let oc = ray.origin() - center_of_sphere;
    // now we have a quadratic equation which we're getting the coefficients for:
    // ax^2 + bx + c
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    return discriminant > 0.0;
}

// for now, this function is the sole decider of the colours in the scene.
fn ray_colour(ray: Ray) -> Rgb<u8> {
    // here we define sphere of size 0.5 and at position -1 from camera in mid.
    if sphere_intersect(Vector3::new(0.0, 0.0, -1.0), 0.5, ray.clone()) {
        return Rgb([255, 0, 0]); // if intersect then sphere is red
    }
    let norm_direction = ray.direction().normalize();
    let t = 0.5 * (norm_direction.y + 1.0);
    // lerp the 2 colours sky blue and white
    let colour_vec = (t) * Vector3::new(1.0, 1.0, 1.0) + (1.0 - t) * Vector3::new(0.5, 0.7, 1.0);
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

    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for y in (0..IMAGE_HEIGHT as u32).rev() {
        for x in 0..IMAGE_WIDTH as u32 {
            let u = x as f64 / (IMAGE_WIDTH - 1.0);
            let v = y as f64 / (IMAGE_HEIGHT - 1.0);
            let out_ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            img.put_pixel(x, y, ray_colour(out_ray));
        }
    }

    img.save("scene.png").unwrap();
}
