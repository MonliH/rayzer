use rand::Rng;

use crate::vector::Vector3D;
use crate::vector::N;

#[inline]
pub fn random_n() -> N {
    rand::thread_rng().gen()
}

#[inline]
pub fn random_range(start: N, end: N) -> N {
    start + (end - start) * random_n()
}

#[inline]
pub fn random_int_range(start: usize, end: usize) -> usize {
    rand::thread_rng().gen_range(start..end)
}

#[inline]
pub fn clamp(x: N, start: N, end: N) -> N {
    if x < start {
        start
    } else if x > end {
        end
    } else {
        x
    }
}

#[inline]
pub fn random_in_unit_sphere() -> Vector3D {
    loop {
        let p = Vector3D::random_range(-1.0, 1.0);
        if p.length_sq() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vector3D {
    random_in_unit_sphere().unit()
}

#[inline]
pub fn random_in_unit_disk() -> Vector3D {
    loop {
        let p = Vector3D::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.length_sq() < 1.0 {
            return p;
        }
    }
}
