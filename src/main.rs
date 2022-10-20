mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod camera;

use vec3::Vec3;
use color::write_color;
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;

// image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 1280;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;
const SAMPLES_PER_PIXEL: f32 = 100.0;   

// camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f32 = 1.0;

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, std::f32::INFINITY) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let camera: Camera = Camera::new(VIEWPORT_HEIGHT, VIEWPORT_WIDTH, FOCAL_LENGTH);
    let mut rng = rand::thread_rng();
    
    let world = HittableList::new(
        vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5
            )),
            Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0
            ))
        ]
    );

    // render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);

    for j in (0..IMAGE_HEIGHT).rev() {
        
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL as i32 {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH-1) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT-1) as f32;
                let ray: Ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}
