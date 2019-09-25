use crate::vec3::Vec3;
use crate::ray::Ray;

pub(crate) struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub(crate) fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
        Camera {lower_left_corner, horizontal, vertical, origin}
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner +
                self.horizontal * u +
                self.vertical * v -
                self.origin
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3(-2., -1., -1.),
            Vec3(4., 0., 0.),
            Vec3(0., 2., 0.),
            Vec3(0., 0., 0.)
        )
    }
}
