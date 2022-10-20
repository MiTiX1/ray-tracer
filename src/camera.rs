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
    pub fn new(viewport_height: f32, viewport_width: f32, focal_length: f32) -> Self {
        
        let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length)
        }
    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}