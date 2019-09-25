use std::ops::{Add, Sub, Mul, Div};

#[derive(PartialEq,Debug,Clone,Copy)]
pub(crate) struct Vec3(pub(crate) f64, pub(crate) f64, pub(crate) f64);

impl Vec3 {
    pub(crate) fn x(&self) -> f64 {
        self.0
    }
    pub(crate) fn y(&self) -> f64 {
        self.1
    }
    pub(crate) fn z(&self) -> f64 {
        self.2
    }
    pub(crate) fn r(&self) -> f64 {
        self.0
    }
    pub(crate) fn g(&self) -> f64 {
        self.1
    }
    pub(crate) fn b(&self) -> f64 {
        self.2
    }
    pub(crate) fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub(crate) fn squared_length(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub(crate) fn unit_vector(&self) -> Self {
        *self / self.length()
    }
    pub(crate) fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 +
        self.1 * rhs.1 +
        self.2 * rhs.2
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self (self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self (self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self (self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self (self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self (self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self (self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Default for Vec3 {
    fn default() -> Self {
        Self(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(Vec3(1., 2., 3.) + Vec3(1., 2., 3.), Vec3(2., 4., 6.))
    }
    #[test]
    fn test_vec3_sub() {
        assert_eq!(Vec3(1., 2., 3.) - Vec3(1., 2., 3.), Vec3(0., 0., 0.))
    }
    #[test]
    fn test_vec3_mul() {
        assert_eq!(Vec3(1., 2., 3.) * Vec3(1., 2., 3.), Vec3(1., 4., 9.));
        assert_eq!(Vec3(1., 2., 3.) * 10., Vec3(10., 20., 30.));
    }
    #[test]
    fn test_vec3_div() {
        assert_eq!(Vec3(1., 2., 3.) / Vec3(1., 2., 10.), Vec3(1., 1., 0.3));
        assert_eq!(Vec3(1., 2., 3.) / 10., Vec3(0.1, 0.2, 0.3));
    }
    #[test]
    fn test_vec3_length() {
        assert_eq!(Vec3(1., 2., 2.).length(), 3.);
    }
    #[test]
    fn test_vec3_dot() {
        assert_eq!(Vec3(2., 2., 1.).dot(Vec3(1., 1., 1.)), 5.);
    }
    #[test]
    fn test_vec3_color() {
        let v = Vec3(1., 2., 3.);
        assert_eq!(1., v.r());
        assert_eq!(2., v.g());
        assert_eq!(3., v.b());
    }
    #[test]
    fn test_vec3_point() {
        let v = Vec3(1., 2., 3.);
        assert_eq!(1., v.x());
        assert_eq!(2., v.y());
        assert_eq!(3., v.z());
    }
}
