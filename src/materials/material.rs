use std::sync::Arc;

use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::vector::Color;

pub type SharedMaterial = Arc<dyn Material + Sync + Send>;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
