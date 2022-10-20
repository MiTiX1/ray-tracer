mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;

use vec3::Vec3;
use color::write_color;
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;

// image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 1280;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;

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
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    
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
            let u: f32 = i as f32 / (IMAGE_WIDTH-1) as f32;
            let v: f32 = j as f32 / (IMAGE_HEIGHT-1) as f32;
            let ray: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&ray, &world);
            
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
