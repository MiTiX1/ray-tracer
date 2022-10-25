use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        
        let theta: f32 = vfov * std::f32::consts::PI / 180.0;
        let h: f32 = (theta/2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3 = Vec3::unit_vector(&(*lookfrom - *lookat));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);

        let origin: Vec3 = *lookfrom;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w, 
        }
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}