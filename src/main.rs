mod vectors;
mod color;
mod ray;
mod hits;
mod spheres;
mod rtweekend;
mod intervals;
mod camera;
mod material;
use crate::color::Color;
use crate::material::Material;
use crate::rtweekend::random_f64;
use crate::rtweekend::random_f64_range;
use crate::vectors::*;
use crate::hits::*;
use crate::spheres::*;
use crate::camera::*;

fn main() {
    // TODO: write directly to a file rather than stdout

    // World
    let mut world: HittableList<Sphere> = HittableList::new();

    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Material::Lambertian(albedo)
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    Material::Metal(albedo, fuzz)
                } else {
                    // Glass
                    Material::Dialectric(1.5)
                };

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Material::Dialectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let mut cam: Camera = Camera::defaults();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; 
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Point3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
