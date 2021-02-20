use crate::hittables::{HitRecord, Hittable};
use crate::vector::{Color, Point3D, Vector3D, N};

#[derive(Copy, Clone, Default)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
    time: N,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D, time: N) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline]
    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    #[inline]
    pub fn direction(&self) -> &Vector3D {
        &self.direction
    }

    #[inline]
    pub fn time(&self) -> &N {
        &self.time
    }

    #[inline]
    pub fn at(&self, t: N) -> Point3D {
        &self.origin + &(&self.direction * t)
    }

    pub fn color(&self, world: &dyn Hittable, depth: usize) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord::default();
        if world.hit(self, 0.001, N::MAX, &mut hit_record) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            let material = hit_record.material.clone();
            if material.scatter(&self, &mut hit_record, &mut attenuation, &mut scattered) {
                attenuation * scattered.color(world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_dir = self.direction.unit();
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}
