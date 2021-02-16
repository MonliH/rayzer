use crate::ray::Ray;
use crate::utils;
use crate::vector::{Point3D, Vector3D, N};

pub struct Camera {
    horizontal: Vector3D,
    vertical: Vector3D,
    lower_left_corner: Point3D,
    origin: Point3D,
    lens_radius: N,
    u: Vector3D,
    v: Vector3D,
}

impl Camera {
    pub fn new(
        lookfrom: Point3D,
        lookat: Point3D,
        vup: Vector3D,
        vfov: N,
        aspect_ratio: N,
        aperature: N,
        focus_distance: N,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperature / 2.0;

        Self {
            horizontal,
            vertical,
            origin,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    #[inline]
    pub fn get_ray(&self, s: N, t: N) -> Ray {
        let rd = utils::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * *rd.x() + self.v * *rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
