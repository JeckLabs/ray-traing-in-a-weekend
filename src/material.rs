use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hit;

pub(crate) trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &Vec3) -> Option<Ray>;
}

pub(crate) struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub(crate) fn new(albedo: Vec3) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &Vec3) -> Option<Ray> {
        unimplemented!()
    }
}

