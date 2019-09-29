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
use std::sync::Arc;
use crate::material::{Metal, Dielectric};

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


fn color(r: &Ray, world: &HittableCollection, depth: i32) -> Vec3 {
    if depth > 50 {
        return Vec3::default()
    }
    if let Some(hit) = world.hit(r, 0.001, std::f64::MAX) {
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::default()
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t
    }
}

fn random_scene() -> HittableCollection {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

    // ground
    objects.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5)))
    )));

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat = rng.next_f64();
            let center = Vec3(a as f64 + 0.9 * rng.next_f64(), 0.2, b as f64 + 0.9 * rng.next_f64());
            let distance = (center - Vec3(4., 0.2, 0.)).length();
            if distance < 0.9 {
                continue;
            }
            if chose_mat < 0.8 { // diffuse
                let color = Vec3(rng.next_f64() * rng.next_f64(), rng.next_f64() * rng.next_f64(), rng.next_f64() * rng.next_f64());
                objects.push(Box::new(Sphere::new(center, 0.2, Arc::new(Lambertian::new(color)))));
            } else if chose_mat < 0.95 { // metal
                let color = Vec3(0.5 * (1. + rng.next_f64()), 0.5 * (1. + rng.next_f64()), 0.5 * (1. + rng.next_f64()));
                let fuzz = 0.5 * rng.next_f64();
                objects.push(Box::new(Sphere::new(center, 0.2, Arc::new(Metal::new(color, fuzz)))));
            } else { // glass
                objects.push(Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        Vec3(0., 1., 0.),
        1.,
        Arc::new(Dielectric::new(1.5))
    )));
    objects.push(Box::new(Sphere::new(
        Vec3(-4., 1., 0.),
        1.,
        Arc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)))
    )));
    objects.push(Box::new(Sphere::new(
        Vec3(4., 1., 0.),
        1.,
        Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.))
    )));

    HittableCollection::new(objects)
}

fn main() {
    let path = Path::new("image.ppm");

    let mut file = match File::create(&path) {
        Err(e) => panic!("fail to create {}: {}", path.display(), e),
        Ok(f) => f,
    };

    let world = random_scene();

    let nx = 1200;
    let ny = 800;
    let ns = 100;

    let look_from = Vec3(13., 2., 3.);
    let look_at = Vec3(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let aspect = nx as f64 / ny as f64;

    let cam = Camera::new(look_from, look_at, Vec3(0., 1., 0.), 20., aspect, aperture, dist_to_focus);

    write!(file, "P3\n{} {}\n255\n", nx, ny).expect("Fail to write");

    let now = precise_time_ns();
    let mut sm: SplitMix64 = SeedableRng::from_seed(now);
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

    for j in 0..ny {
        println!("\033[1ALine: {}", j + 1);
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let v = ((ny - j) as f64 + rng.next_f64()) / ny as f64;
                let u = (i as f64 + rng.next_f64()) / nx as f64;
                let r = cam.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }

            col = col / ns as f64;
            let ir = (col.r().sqrt() * 255.99) as u8;
            let ig = (col.g().sqrt() * 255.99) as u8;
            let ib = (col.b().sqrt() * 255.99) as u8;

            write!(file, "{} {} {}\n", ir, ig, ib).expect("Fail to write");
        }
    }
}
