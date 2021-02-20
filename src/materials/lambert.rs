use super::Material;
use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use crate::vector::Color;

pub struct Lambert(Color);

impl Lambert {
    pub fn new(albedo: Color) -> Self {
        Self(albedo)
    }
}

impl Material for Lambert {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.p, scatter_direction, *r_in.time());
        *attenuation = self.0;
        true
    }
}
