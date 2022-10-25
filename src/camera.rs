use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        
        let theta: f32 = vfov * std::f32::consts::PI / 180.0;
        let h: f32 = (theta/2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3 = Vec3::unit_vector(&(*lookfrom - *lookat));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);

        let origin: Vec3 = *lookfrom;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            _w: w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}