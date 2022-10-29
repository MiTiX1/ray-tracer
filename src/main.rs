mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use vec3::Vec3;
use color::write_color;
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;
use material::{Metal, Lambertian, Dielectric};
use rayon::prelude::*;

// image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 1280;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;
const SAMPLES_PER_PIXEL: f32 = 100.0; 
const MAX_DEPTH: i32 = 50;

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, std::f32::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(&ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let material_ground: Lambertian = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let mut world: HittableList = HittableList::new(
        vec![
            Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                material_ground
            )),
        ]
    );

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center: Vec3 = Vec3::new(
                a as f32 + 0.9*rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9*rng.gen::<f32>()
            );

            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Vec3 = Vec3::random() * Vec3::random();
                    let sphere_mat: Lambertian = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_mat
                    )));
                } else if choose_mat < 0.95 {
                    let albedo: Vec3 = Vec3::random_min_max(0.5, 1.0);
                    let fuzz: f32 = rng.gen_range(0.0..0.5);
                    let sphere_mat: Metal = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_mat
                    )));
                } else {
                    let sphere_mat: Dielectric = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_mat
                    )));
                }
            }
        }
    }
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5)
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1))
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)
    )));

    world
}

fn main() {
    let lookfrom: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let lookat: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera: Camera = Camera::new(
        &lookfrom,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        20.0, 
        ASPECT_RATIO,
        0.1,
        10.0
    );

    let world: HittableList = random_scene();

    // render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);

    for j in (0..IMAGE_HEIGHT).rev() {
        
        eprintln!("Scanlines remaining: {}", j);

        let scanlines: Vec<Vec3> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL as i32 {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH-1) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT-1) as f32;
                let ray: Ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            pixel_color
        }).collect();
        for color in scanlines {
            write_color(color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}
