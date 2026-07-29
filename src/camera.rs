use crate::rtweekend::*;
use crate::vectors::*;
use crate::color::*;
use crate::hits::*;
use crate::ray::*;
use crate::intervals::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    image_height: i32,          // Rendered image height
    pixel_samples_scale: f64,   // Color scale factor for a sum of pixel samples
    center: Point3,             // Camera Center 
    pixel00_loc: Vec3,          // Location of pizel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
}

impl Camera {
    pub fn defaults() -> Camera {
        Camera { 
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::zero(),
            pixel00_loc: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            let scanline = self.image_height - j;
            eprint!("\rScanlines remaining: {scanline} ");
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        eprint!("\rDone.                            \n");
    }

    fn initialize(&mut self) {
        // Caluclate the image height, and ensure that it's at least 1.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = Point3::zero();

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height as f64));

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directeed at randomly sampled
        // point arround the pixel location i, j.
        
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-0.5,-0.5] - [+0.5,+0.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() + 0.5, 0.0)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &impl Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec: HitRecord = HitRecord::new(); 

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth-1, world);
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = 0.5*(unit_direction.y() + 1.0);
        (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}
