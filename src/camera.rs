use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f64::consts::PI;
use time::precise_time_ns;
use xorshift::{SeedableRng, Rand, SplitMix64, Xoroshiro128, Rng};

pub(crate) struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

    loop {
        p = Vec3(rng.next_f64(), rng.next_f64(), 0.) * 2. - Vec3(1., 1., 0.,);
        if p.dot(p) < 1. {
            return p
        }
    }
}

impl Camera {
    pub(crate) fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let lens_radius = aperture / 2.;
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = half_height * aspect;
        let origin = look_from;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;

        let horizontal = u * half_width * focus_dist * 2.;
        let vertical = v * half_height * focus_dist * 2.;

        Camera {lower_left_corner, horizontal, vertical, origin, lens_radius, u, v}
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rn = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rn.x() + self.v * rn.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner +
                self.horizontal * s +
                self.vertical * t -
                self.origin -
                offset
        )
    }
}
