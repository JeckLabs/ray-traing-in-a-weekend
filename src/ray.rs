use crate::vec3::Vec3;

pub(crate) struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub(crate) fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a, b}
    }
    pub(crate) fn origin(&self) -> Vec3 {
        self.a
    }
    pub(crate) fn direction(&self) -> Vec3 {
        self.b
    }
    pub(crate) fn point_at(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
