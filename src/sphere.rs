use crate::{
    vec3::Vec3, 
    hittable::Hittable, 
    hittable::HitRecord, 
    ray::Ray,
    material::{Material},
};

pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f32,
    material: M
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Self {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().length_squared();
        let half_b: f32 = Vec3::dot(&oc, &ray.direction());
        let c: f32 = oc.length_squared() - self.radius.powi(2);

        let discriminant: f32 = half_b.powi(2) - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f32 = discriminant.sqrt();
        let mut root: f32 = (-half_b-sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b+sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        } 

        let p = ray.at(root);

        Some(HitRecord {
            t: root,
            p,
            normal: (p - self.center) / self.radius,
            front_face: true,
            material: &self.material
        })
    }
}