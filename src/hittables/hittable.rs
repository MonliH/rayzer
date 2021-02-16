use std::sync::Arc;

use crate::materials::{Lambert, SharedMaterial};
use crate::ray::Ray;
use crate::vector::{Color, Point3D, Vector3D, N};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vector3D,
    pub t: N,
    pub front_face: bool,
    pub material: SharedMaterial,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3D::default(),
            normal: Vector3D::default(),
            t: N::default(),
            front_face: false,
            material: Arc::new(Lambert::new(Color::new(0.0, 0.0, 0.0))),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3D) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool;
}

pub struct Hittables(Vec<Box<dyn Hittable + Sync + Send>>);
impl Hittables {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, item: Box<dyn Hittable + Sync + Send>) {
        self.0.push(item);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool {
        let mut new_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.0 {
            if object.hit(ray, t_min, closest_so_far, &mut new_rec) {
                hit_anything = true;
                closest_so_far = new_rec.t;
                *rec = new_rec.clone();
            }
        }

        hit_anything
    }
}
