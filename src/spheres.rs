use crate::vectors::*;
use crate::ray::*;
use crate::hits::*;
use crate::intervals::*;
use crate::material::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Sphere {
        Sphere { 
            center: center,
            radius: radius.max(0.0),
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 { return false; }

        let sqrtd = discriminant.sqrt();
        
        // Find the nearest rot that lies on the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat;
        
        return true;
    }
}
