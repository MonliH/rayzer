use super::{HitRecord, Hittable, AABB};
use crate::materials::SharedMaterial;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D, N};

#[derive(Clone)]
pub struct MovingSphere {
    center0: Point3D,
    center1: Point3D,
    radius: N,
    material: SharedMaterial,
    time_0: N,
    time_1: N,
}

impl MovingSphere {
    pub fn new(
        center0: Point3D,
        center1: Point3D,
        time_0: N,
        time_1: N,
        radius: N,
        material: SharedMaterial,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time_0,
            time_1,
        }
    }

    fn center(&self, time: &N) -> Point3D {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - &self.center(ray.time());
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
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.clone();

        true
    }

    fn bounding_box(&self, time0: N, time1: N, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(&time0) - Vector3D::new(self.radius, self.radius, self.radius),
            self.center(&time0) + Vector3D::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(&time1) - Vector3D::new(self.radius, self.radius, self.radius),
            self.center(&time1) + Vector3D::new(self.radius, self.radius, self.radius),
        );
        *output_box = box0.surrounding_box(&box1);
        true
    }
}
