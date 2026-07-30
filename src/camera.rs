use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use crate::rtweekend::*;
use crate::vectors::*;
use crate::color::*;
use crate::hits::*;
use crate::ray::*;
use crate::intervals::*;

pub struct Camera {
    pub render_threads: i32,    // Number of worker threads used for rendering

    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count 
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene

    pub vfov: f64,              // Vertical view angle (field of view)
    pub lookfrom: Point3,       // Point Camera is looking from
    pub lookat: Point3,         // Point Camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction

    pub defocus_angle: f64,     // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera lookfrom point to plane of perfect focus

    image_height: i32,          // Rendered image height
    pixel_samples_scale: f64,   // Color scale factor for a sum of pixel samples
    center: Point3,             // Camera Center 
    pixel00_loc: Vec3,          // Location of pizel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
    u: Vec3,                    // Camera Frame Basis vectors 
    v: Vec3, 
    w: Vec3,               
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn defaults() -> Camera {
        Camera { 
            render_threads: 1,

            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::zero(),
            pixel00_loc: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }

    pub fn render_scanline(&self, row: i32, world: Arc<impl Hittable + Send + Sync>) -> Vec<Color> {
        let mut scanline = vec![];
        for i in 0..self.image_width{
            let mut pixel_color = Color::zero();
            for _sample in 0..self.samples_per_pixel {
                let r: Ray = self.get_ray(i, row);
                pixel_color += self.ray_color(&r, self.max_depth, world.clone());
            }
             scanline.push(self.pixel_samples_scale * pixel_color);
        }
        return scanline;
    }

    pub fn render<W: Hittable + Send + Sync + 'static>(mut self, world: W) {
        self.initialize();

        let fifo = Arc::new(Mutex::new(VecDeque::new()));
        let thread_camera = Arc::new(self);
        let thread_world = Arc::new(world);
        let mut handles = vec![];
        let mut recievers = vec![];

        for _ in 0..thread_camera.render_threads {
            let fifo = Arc::clone(&fifo);
            let my_cam = Arc::clone(&thread_camera);
            let my_world = Arc::clone(&thread_world);

            let (tx, rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                loop {
                    let mut work_list = fifo.lock().unwrap();
                    match work_list.pop_front() { 
                        Some(row) => {
                            drop(work_list);
                            let done = my_cam.render_scanline(row, my_world.clone());
                            match tx.send((done, row)) {
                               Err(_) => {
                                   break;
                               }
                               _ => {}
                            }
                        }
                        _ => { drop(work_list); }
                    }
                }
            });

            recievers.push(rx);
            handles.push(handle);
        }

        for row in 0..thread_camera.image_height {
            let mut work_list = fifo.lock().unwrap();
            work_list.push_back(row);
            drop(work_list)
        }

        let mut image = vec![vec![Color::zero(); thread_camera.image_width as usize]; thread_camera.image_height as usize];

        let mut done = 0;
        while done < thread_camera.image_height {
            eprint!("\rScanlines remaining: {} ", thread_camera.image_height - done);
            for rx in &recievers {
                match rx.try_recv() {
                    Ok((scanline, row)) => {
                        image[row as usize] = scanline;
                        done += 1;
                    }
                    _ => {}
                }
            }
        }
        eprint!("\rDone.                            \n");
        drop(recievers);

        /*
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                self.image[j as usize][i as usize] = self.pixel_samples_scale * pixel_color;
            }
        }
        */

        println!("P3\n{} {}\n255", thread_camera.image_width, thread_camera.image_height);
        for scanline in &image {
            for pixel in scanline {
                write_color(&pixel);
            }
        }
    }

    fn initialize(&mut self) {
        // Caluclate the image height, and ensure that it's at least 1.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height as f64));

        // Calculate the u,v,w unit basis vectors fro the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w);
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - (self.focus_dist*self.w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directeed at randomly sampled
        // point arround the pixel location i, j.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-0.5,-0.5] - [+0.5,+0.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() + 0.5, 0.0)
    }
    
    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk;
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: Arc<impl Hittable + Send + Sync>) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec: HitRecord = HitRecord::new(); 

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered: Ray = Ray::zero();
            let mut attenuation: Color = Color::zero();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth-1, world)
            } else {
                return Color::zero();
            }
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = 0.5*(unit_direction.y() + 1.0);
        (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}
