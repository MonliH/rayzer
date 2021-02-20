use super::Material;
use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::utils;
use crate::vector::{Color, N};

pub struct Metal {
    albedo: Color,
    roughness: N,
}

impl Metal {
    pub fn new(albedo: Color, roughness: N) -> Self {
        Self {
            albedo,
            roughness: utils::clamp(roughness, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = super::reflect(&r_in.direction().unit(), &hit_record.normal);
        *scattered = Ray::new(
            hit_record.p,
            reflected + utils::random_in_unit_sphere() * self.roughness,
            *r_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction().dot(&hit_record.normal) > 0.0
    }
}
