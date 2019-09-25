use std::path::Path;
use std::fs::File;
use std::io::Write;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian};
use crate::hittable::{HittableCollection, Hittable};

extern crate time;
extern crate xorshift;

use time::precise_time_ns;
use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128};

fn random_in_unit_sphere() -> Vec3 {
    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);
    let ones = Vec3(1., 1., 1.);
    let mut p;
    loop {
        p = Vec3(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 2. - ones;
        if p.squared_length() < 1. {
            break
        }
    }
    p
}


fn color(r: &Ray, world: &HittableCollection) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f64::MAX) {
        let target = hit.p + hit.normal + random_in_unit_sphere();
//        let nu = hit.normal.unit_vector();
//        Vec3(nu.x() + 1., nu.y() + 1., nu.z() + 1.) * 0.5
        color(&Ray::new(hit.p, target - hit.p), world) * 0.5
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t
    }
}

fn main() {
    let path = Path::new("image.ppm");

    let mut file = match File::create(&path) {
        Err(e) => panic!("fail to create {}: {}", path.display(), e),
        Ok(f) => f,
    };

    let world = HittableCollection::new(
        vec!(
            Box::new(Sphere::new(
                Vec3(0., 0., -1.),
                0.5,
                Lambertian::new(Vec3::default())
            )),
            Box::new(Sphere::new(
                Vec3(0., -100.5, -1.),
                100.,
                Lambertian::new(Vec3::default())
            )),
        )
    );

    let nx = 200;
    let ny = 100;
    let ns = 100;

    let cam = Camera::default();

    write!(file, "P3\n{} {}\n255\n", nx, ny).expect("Fail to write");

    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

    for j in 0..ny {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let v = ((ny - j) as f64 + rng.next_f64()) / ny as f64;
                let u = (i as f64 + rng.next_f64()) / nx as f64;
                let r = cam.get_ray(u, v);
                col = col + color(&r, &world);
            }

            col = col / ns as f64;
            let ir = (col.r().sqrt() * 255.99) as u8;
            let ig = (col.g().sqrt() * 255.99) as u8;
            let ib = (col.b().sqrt() * 255.99) as u8;

            write!(file, "{} {} {}\n", ir, ig, ib).expect("Fail to write");
        }
    }
}
