use crate::vector::{Vector3D, N};

pub fn reflect(v: &Vector3D, normal: &Vector3D) -> Vector3D {
    normal - &(normal * v.dot(normal) * 2.0)
}

pub fn refract(uv: &Vector3D, n: &Vector3D, etai_over_etat: N) -> Vector3D {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = (uv + &(n * cos_theta)) * etai_over_etat;
    let r_out_parallel = -n * (1.0 - r_out_perp.length_sq()).abs().sqrt();
    r_out_perp + r_out_parallel
}
