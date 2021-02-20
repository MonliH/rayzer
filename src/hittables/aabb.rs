use super::HitRecord;
use crate::ray::Ray;
use crate::vector::{Point3D, N};

#[derive(Clone, Default)]
pub struct AABB {
    min: Point3D,
    max: Point3D,
}

impl AABB {
    pub fn new(min: Point3D, max: Point3D) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Point3D {
        self.min
    }

    pub fn max(&self) -> Point3D {
        self.max
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let small = Point3D::new(
            N::min(*self.min.x(), *other.min.x()),
            N::min(*self.min.y(), *other.min.y()),
            N::min(*self.min.z(), *other.min.z()),
        );

        let big = Point3D::new(
            N::max(*self.max.x(), *other.max.x()),
            N::max(*self.max.y(), *other.max.y()),
            N::max(*self.max.z(), *other.max.z()),
        );

        Self::new(small, big)
    }

    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: N, t_max: N, _: &mut HitRecord) -> bool {
        let dims = [
            (
                self.min.x(),
                self.max.x(),
                ray.origin().x(),
                ray.direction().x(),
            ),
            (
                self.min.y(),
                self.max.y(),
                ray.origin().y(),
                ray.direction().y(),
            ),
            (
                self.min.z(),
                self.max.z(),
                ray.origin().z(),
                ray.direction().z(),
            ),
        ];
        for (min, max, origin, dir) in &dims {
            let inv_direction = 1.0 / *dir;
            let mut t0 = (*min - *origin) * inv_direction;
            let mut t1 = (*max - *origin) * inv_direction;
            if inv_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let new_min = if t0 > t_min { t0 } else { t_min };
            let new_max = if t1 < t_max { t1 } else { t_max };
            if new_max <= new_min {
                return false;
            }
        }
        true
    }
}
