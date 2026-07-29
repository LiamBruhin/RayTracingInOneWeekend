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
use crate::vectors::*;
use crate::hits::*;
use crate::spheres::*;
use crate::camera::*;

fn main() {
    // World
    let mut world: HittableList<Sphere> = HittableList::new();

    let material_ground: Material = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center : Material = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left : Material = Material::Metal(Color::new(0.8, 0.8, 0.8));
    let material_right : Material = Material::Metal(Color::new(0.8, 0.6, 0.2));

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));
    
    // Camera
    let mut cam: Camera = Camera::defaults();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; 
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
