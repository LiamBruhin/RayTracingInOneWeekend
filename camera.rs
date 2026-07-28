use rtweekend::*;
use vectors::*;
use color::*;
use hits::*;
use ray::*;
use intervals::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    image_height: i32,      // Rendered image height
    center: Point3,         // Camera Center 
    pixel00_loc: Vec3,      // Location of pizel 0, 0
    pixel_delta_u: Vec3,    // Offset to pixel to the right
    pixel_delta_v: Vec3,    // Offset to pixel below
}

impl Camera {
    pub fn defaults() -> Camera {
        Camera { 
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
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
                let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray::new(self.center, ray_direction);

                let pixel_color: Color = self.ray_color(&r, world);
                write_color(pixel_color);
            }
        }
        eprint!("\rDone.                            \n");
    }

    fn initialize(&mut self) {
        // Caluclate the image height, and ensure that it's at least 1.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

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

    fn ray_color(&self, r: &Ray, world: &impl Hittable) -> Color {
        let mut rec: HitRecord = HitRecord::new(); 

        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = 0.5*(unit_direction.y() + 1.0);
        return (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
    }
}
