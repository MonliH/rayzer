use super::{HitRecord, Hittable, AABB};
use crate::materials::SharedMaterial;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D, N};

#[derive(Clone)]
pub struct Sphere {
    center: Point3D,
    radius: N,
    material: SharedMaterial,
}

impl Sphere {
    pub fn new(center: Point3D, radius: N, material: SharedMaterial) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_sq();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_sq() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;

        if discrim < 0.0 {
            return false;
        }

        let sqrt_d = N::sqrt(discrim);
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.clone();

        true
    }

    fn bounding_box(&self, _: N, _: N, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vector3D::new(self.radius, self.radius, self.radius),
            self.center + Vector3D::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
