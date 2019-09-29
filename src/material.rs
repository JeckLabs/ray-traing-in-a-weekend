use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hit;
use crate::random_in_unit_sphere;
use xorshift::{SeedableRng, SplitMix64, Xoroshiro128, Rng, Rand};
use time::precise_time_ns;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub(crate) trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

pub(crate) struct Lambertian {
    albedo: Vec3,
}
pub(crate) struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
pub(crate) struct Dielectric {
    ref_idx: f64,
}

impl Lambertian {
    pub(crate) fn new(albedo: Vec3) -> Self {
        Self {albedo}
    }
}
impl Metal {
    pub(crate) fn new(albedo: Vec3, fuzz: f64) -> Self {
        if fuzz < 0. {
            Self {albedo, fuzz: 0.}
        } else if fuzz > 1. {
            Self {albedo, fuzz: 1.}
        } else {
            Self {albedo, fuzz}
        }
    }
}
impl Dielectric {
    pub(crate) fn new(ref_idx: f64) -> Self {
        Self {ref_idx}
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let mut r0 = (1. - self.ref_idx) / (1. + self.ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction().unit_vector(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere() * self.fuzz);

        if scattered.direction().dot(hit.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(hit.p, target - hit.p)))
    }
}

fn get_rand() -> f64 {
    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);
    rng.next_f64()
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3(1., 1., 1.);

        let outward_normal: Vec3;
        let ni_over_nt: f64;
        let cosine: f64;

        let reflected = reflect(r_in.direction(), hit.normal);

        if r_in.direction().dot(hit.normal) > 0. {
            outward_normal = hit.normal * -1.;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction().dot(hit.normal) / r_in.direction().length()
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1. / self.ref_idx;
            cosine = -1. * r_in.direction().dot(hit.normal) / r_in.direction().length()
        }

        if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
            if get_rand() > self.schlick(cosine) {
                return Some((attenuation, Ray::new(hit.p,refracted)))
            }
        }

        Some((attenuation, Ray::new(hit.p,reflected)))
    }
}

