use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::{random_n, random_range};

pub type N = f64;
pub type Point3D = Vector3D;
pub type Color = Vector3D;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vector3D(N, N, N);

impl Vector3D {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Self(x, y, z)
    }

    #[inline]
    pub fn x(&self) -> &N {
        &self.0
    }

    #[inline]
    pub fn y(&self) -> &N {
        &self.1
    }

    #[inline]
    pub fn z(&self) -> &N {
        &self.2
    }

    #[inline]
    pub fn random() -> Self {
        Self(random_n(), random_n(), random_n())
    }

    #[inline]
    pub fn random_range(min: N, max: N) -> Self {
        Self(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    #[inline]
    /// Euclidean distance
    pub fn length(&self) -> N {
        N::sqrt(self.length_sq())
    }

    #[inline]
    pub fn length_sq(&self) -> N {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Cross product
    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> N {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn unit(&self) -> Self {
        let len = self.length();
        self / len
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
}

// Scalar multiplication
impl Mul<N> for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn mul(self, other: N) -> Vector3D {
        Vector3D(self.0 * other, self.1 * other, self.2 * other)
    }
}

// Scalar division
impl Div<N> for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn div(self, other: N) -> Vector3D {
        Vector3D(self.0 / other, self.1 / other, self.2 / other)
    }
}

// Scalar subtraction
impl Sub<N> for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn sub(self, other: N) -> Vector3D {
        Vector3D(self.0 - other, self.1 - other, self.2 - other)
    }
}

// Scalar addition
impl Add<N> for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn add(self, other: N) -> Vector3D {
        Vector3D(self.0 + other, self.1 + other, self.2 + other)
    }
}

// Negate each element
impl Neg for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn neg(self) -> Vector3D {
        Vector3D(-self.0, -self.1, -self.2)
    }
}

// Element-wise multiplication
impl Div for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn div(self, other: Self) -> Vector3D {
        Vector3D(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

// Element-wise multiplication
impl Mul for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn mul(self, other: Self) -> Vector3D {
        Vector3D(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

// Element-wise addition
impl Add for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn add(self, other: Self) -> Vector3D {
        Vector3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// Element-wise subtraction
impl Sub for &Vector3D {
    type Output = Vector3D;

    #[inline]
    fn sub(self, other: Self) -> Vector3D {
        Vector3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// Scalar multiplication
impl Mul<N> for Vector3D {
    type Output = Self;

    #[inline]
    fn mul(self, other: N) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

// Scalar division
impl Div<N> for Vector3D {
    type Output = Self;

    #[inline]
    fn div(self, other: N) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

// Scalar subtraction
impl Sub<N> for Vector3D {
    type Output = Self;

    #[inline]
    fn sub(self, other: N) -> Self {
        Self(self.0 - other, self.1 - other, self.2 - other)
    }
}

// Scalar addition
impl Add<N> for Vector3D {
    type Output = Self;

    #[inline]
    fn add(self, other: N) -> Self {
        Self(self.0 + other, self.1 + other, self.2 + other)
    }
}

// Negate each element
impl Neg for Vector3D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

// Element-wise multiplication
impl Div for Vector3D {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

// Element-wise multiplication
impl Mul for Vector3D {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

// Element-wise addition
impl Add for Vector3D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// Element-wise subtraction
impl Sub for Vector3D {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// Scalar adding assign
impl AddAssign<N> for Vector3D {
    #[inline]
    fn add_assign(&mut self, other: N) {
        self.0 += other;
        self.1 += other;
        self.2 += other;
    }
}

// Scalar subtracting assign
impl SubAssign<N> for Vector3D {
    #[inline]
    fn sub_assign(&mut self, other: N) {
        self.0 -= other;
        self.1 -= other;
        self.2 -= other;
    }
}

// Scalar divide assign
impl DivAssign<N> for Vector3D {
    #[inline]
    fn div_assign(&mut self, other: N) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

// Scalar multiply assign
impl MulAssign<N> for Vector3D {
    #[inline]
    fn mul_assign(&mut self, other: N) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

// Adding assign
impl AddAssign for Vector3D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

// Subtracting assign
impl SubAssign for Vector3D {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

// Divide assign
impl DivAssign for Vector3D {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

// Multiply assign
impl MulAssign for Vector3D {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

#[test]
fn vector_values() {
    let vector = Vector3D::new(0.0, 1.0, 2.0);
    assert_eq!(vector.x(), &0.0);
    assert_eq!(vector.y(), &1.0);
    assert_eq!(vector.z(), &2.0);
}

#[test]
fn vector_add_scalar_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 += 1.0;
    assert_eq!(Vector3D::new(1.1, 1.2, 1.3), vector1);
}

#[test]
fn vector_sub_scalar_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 -= 0.1;
    assert_eq!(Vector3D::new(0.0, 0.1, 0.20000002), vector1);
}

#[test]
fn vector_mul_scalar_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 *= 2.0;
    assert_eq!(Vector3D::new(0.2, 0.4, 0.6), vector1);
}

#[test]
fn vector_div_scalar_assign() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    let mut vector2 = Vector3D::new(0.2, 0.4, 0.6);
    vector2 /= 2.0;
    assert_eq!(vector1, vector2);
}

#[test]
fn vector_add_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 += vector1.clone();
    assert_eq!(Vector3D::new(0.2, 0.4, 0.6), vector1);
}

#[test]
fn vector_sub_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 -= vector1.clone();
    assert_eq!(Vector3D::new(0.0, 0.0, 0.0), vector1);
}

#[test]
fn vector_div_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 /= vector1.clone();
    assert_eq!(Vector3D::new(1.0, 1.0, 1.0), vector1);
}

#[test]
fn vector_mul_assign() {
    let mut vector1 = Vector3D::new(0.1, 0.2, 0.3);
    vector1 *= vector1.clone();
    assert_eq!(Vector3D::new(0.010000001, 0.040000003, 0.09), vector1);
}

#[test]
fn vector_add_scalar() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(1.1, 1.2, 1.3), vector1.clone() + 1.0);
}

#[test]
fn vector_sub_scalar() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(0.0, 0.1, 0.20000002), vector1.clone() - 0.1);
}

#[test]
fn vector_mul_scalar() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(0.2, 0.4, 0.6), vector1 * 2.0);
}

#[test]
fn vector_div_scalar() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(vector1, Vector3D::new(0.2, 0.4, 0.6) / 2.0);
}

#[test]
fn vector_add() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(0.2, 0.4, 0.6), vector1.clone() + vector1);
}

#[test]
fn vector_sub() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(0.0, 0.0, 0.0), vector1.clone() - vector1);
}

#[test]
fn vector_neg() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(-0.1, -0.2, -0.3), -vector1);
}

#[test]
fn vector_div() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(Vector3D::new(1.0, 1.0, 1.0), vector1.clone() / vector1);
}

#[test]
fn vector_mul() {
    let vector1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_eq!(
        Vector3D::new(0.010000001, 0.040000003, 0.09),
        vector1.clone() * vector1
    );
}

#[test]
fn vector_length() {
    assert_eq!(54.0, Vector3D::new(4.0, 28.0, 46.0).length());
}

#[test]
fn vector_length_sq() {
    assert_eq!(2916.0, Vector3D::new(4.0, 28.0, 46.0).length_sq());
}

#[test]
fn dot_product() {
    let vector1 = Vector3D::new(1.0, 2.0, 3.0);
    let vector2 = Vector3D::new(1.0, 5.0, 7.0);
    assert_eq!(Vector3D::new(-1.0, -4.0, 3.0), vector1.cross(&vector2));
}
