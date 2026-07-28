mod vectors;
mod color;
mod ray;
mod hits;
mod spheres;
mod rtweekend;
mod intervals;
mod camera;
use vectors::*;
use hits::*;
use spheres::*;
use camera::*;

fn main() {
    // World
    let mut world: HittableList<Sphere> = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    
    // Camera
    let mut cam: Camera = Camera::defaults();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; 

    cam.render(&world);
}
