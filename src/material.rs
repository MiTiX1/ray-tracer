use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian {
            albedo 
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered: Ray = Ray::new(rec.p, scatter_direction);
        let attenuation: Vec3 = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(&ray.direction()), &rec.normal);

        let scattered: Ray = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        }
        None
    }
}