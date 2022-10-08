mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;

// crate uses
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;

use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;

// std uses
use std::f64;

// for arranging a random scene
fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::default();
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vector3::new(0.5, 0.5, 0.5)),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    ));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vector3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world
}

// for gamma correction
fn ray_colour(colour_non_mapped: Vector3<f64>) -> Rgb<u8> {
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
        if let Some((scattered, attenuation)) = hit.material.scatter(*ray, hit) {
            return attenuation.zip_map(
                &ray_colour_non_manip(&scattered, &world, depth - 1),
                |l, r| l * r,
            );
        }
        Vector3::new(0.0, 0.0, 0.0)
    } else {
        // background (skybox now)
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image specs
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // instead of 16.0 / 9.0
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: u32 = 50;

    // Camera
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // define world and scene
    let world = random_scene();

    // Rendering

    let img: Vec<u8> = (0..IMAGE_HEIGHT as u32)
        .into_iter()
        .rev()
        .flat_map(|j| {
            (0..IMAGE_WIDTH as u32)
                .flat_map(|i| {
                    // cache rand function
                    let col: Vector3<f64> = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let mut rng = rand::thread_rng();

                            let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH;
                            let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT;
                            let ray = cam.get_ray(u, v);
                            ray_colour_non_manip(&ray, &world, MAX_DEPTH)
                        })
                        .sum();
                    col.iter()
                        .map(|c| (255.999 * (c / SAMPLES_PER_PIXEL as f64)) as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    // let col_buffer: Vec<Rgb<u8>> = Vec::new();
    // for colour in img.chunks(3) {
    //     col_buffer.push(ray_colour(Vector3::new(colour[0] as f64, colour[1], colour[2])));
    // }
    if let Some(new_img) = RgbImage::from_vec(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, img) {
        new_img.save("scene.png").unwrap();
    } else {
        eprintln!("Failure in creating final image from Vec<u8>");
    }
}
