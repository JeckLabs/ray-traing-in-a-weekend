use crate::hittable::{Hit, Hittable};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub(crate) struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub(crate) fn new<T: Material + 'static>(center: Vec3, radius: f64, material: T) -> Self {
        Sphere {center, radius, material: Box::new(material)}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at(temp);
                return Some(Hit::new(temp, p, (p - self.center) / self.radius))
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at(temp);
                return Some(Hit::new(temp, p, (p - self.center) / self.radius))
            }
        }
        None
    }
}
