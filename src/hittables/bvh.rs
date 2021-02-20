use std::cmp::Ordering;
use std::sync::Arc;

use super::{HitRecord, Hittable, SharedHittableTraitObj, AABB};
use crate::materials::SharedMaterial;
use crate::ray::Ray;
use crate::utils::random_int_range;
use crate::vector::{Point3D, Vector3D, N};

pub struct BvhNode {
    bounding_box: AABB,
    left: SharedHittableTraitObj,
    right: SharedHittableTraitObj,
}

impl BvhNode {
    fn box_compare(a: SharedHittableTraitObj, b: SharedHittableTraitObj, axis: usize) -> Ordering {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("no bounding box in bvh constructor")
        }

        let min_a = box_a.min();
        let min_b = box_b.min();

        let (dim0, dim1) = match axis {
            0 => (min_a.x(), min_b.x()),
            1 => (min_a.y(), min_b.y()),
            2 => (min_a.z(), min_b.z()),
            _ => unimplemented!(),
        };

        dim0.partial_cmp(dim1).unwrap()
    }

    pub fn new(
        src_objs: &mut [SharedHittableTraitObj],
        start: usize,
        end: usize,
        time0: N,
        time1: N,
    ) -> BvhNode {
        let axis = random_int_range(0, 3);

        let object_span = end - start;
        let (left, right) = if object_span == 1 {
            (Arc::clone(&src_objs[start]), Arc::clone(&src_objs[start]))
        } else if object_span == 2 {
            if Self::box_compare(
                Arc::clone(&src_objs[start]),
                Arc::clone(&src_objs[start + 1]),
                axis,
            ) == Ordering::Less
            {
                (
                    Arc::clone(&src_objs[start]),
                    Arc::clone(&src_objs[start + 1]),
                )
            } else {
                (
                    Arc::clone(&src_objs[start + 1]),
                    Arc::clone(&src_objs[start]),
                )
            }
        } else {
            src_objs[start..end]
                .sort_unstable_by(|a, b| Self::box_compare(Arc::clone(a), Arc::clone(b), axis));
            let mid = start + object_span / 2;
            (
                Arc::new(BvhNode::new(src_objs, start, mid, time0, time1))
                    as SharedHittableTraitObj,
                Arc::new(BvhNode::new(src_objs, mid, end, time0, time1)) as SharedHittableTraitObj,
            )
        };
        let mut box_left = AABB::default();
        let mut box_right = AABB::default();
        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            panic!("no bounding box in bvh constructor");
        }

        let bounding_box = box_left.surrounding_box(&box_right);
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_min, t_max, rec) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, time0: N, time1: N, output_box: &mut AABB) -> bool {
        *output_box = self.bounding_box.clone();
        true
    }
}
