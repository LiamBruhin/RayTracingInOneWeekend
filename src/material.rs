use crate::color::*;
use crate::ray::*;
use crate::hits::*;
use crate::vectors::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    None,
    // Diffuse,
    Lambertian(Color),
    Metal(Color, f64),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Material::Metal(albedo, fuzz) => {
                let normal_fuzz = if *fuzz < 1.0 { *fuzz } else { 1.0 };
                let mut reflected: Vec3 = Vec3::reflect(&r_in.direction(), &rec.normal);
                reflected = reflected.unit_vector() + (normal_fuzz * Vec3::random_unit_vector());
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                scattered.direction().dot(rec.normal) > 0.0
            }
            _ => false,
        }
    }
}
