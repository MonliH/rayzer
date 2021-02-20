use super::Material;
use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::utils::random_n;
use crate::vector::{Color, N};

pub struct Glass {
    ior: N,
}

impl Glass {
    pub fn new(ior: N) -> Self {
        Self { ior }
    }

    fn reflectance(cosine: N, ref_idx: N) -> N {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sine_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sine_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_n() {
                super::reflect(&unit_direction, &hit_record.normal)
            } else {
                super::refract(&unit_direction, &hit_record.normal, refraction_ratio)
            };

        *scattered = Ray::new(hit_record.p, direction, *r_in.time());

        true
    }
}
