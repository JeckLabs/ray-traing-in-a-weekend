use crate::vec3::Vec3;
use crate::ray::Ray;

pub(crate) struct Hit {
    pub(crate) t: f64,
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
}

pub(crate) struct HittableCollection {
    objects: Vec<Box<dyn Hittable>>
}

impl Hit {
    pub(crate) fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
        Hit {t, p, normal}
    }
}

impl HittableCollection {
    pub(crate) fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableCollection {objects}
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl Hittable for HittableCollection {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut res: Option<Hit> = None;
        let mut closest = t_max;
        for obj in self.objects.iter() {
            if let Some(hit) = obj.hit(r, t_min, closest) {
                closest = hit.t;
                res = Some(hit);
            }
        }
        res
    }
}
