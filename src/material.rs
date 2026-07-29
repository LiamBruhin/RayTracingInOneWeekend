use crate::color::*;
use crate::ray::*;
use crate::hits::*;
use crate::rtweekend::random_f64;
use crate::vectors::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    None,
    // Diffuse,
    Lambertian(Color),
    Metal(Color, f64),
    Dialectric(f64),
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
            Material::Dialectric(refraction_index) => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let ri = if rec.front_face { 1.0/(*refraction_index) } else { *refraction_index };

                let unit_direction: Vec3 = r_in.direction().unit_vector();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract: bool = ri * sin_theta > 1.0;
                let direction: Vec3 = if cannot_refract || reflectance(cos_theta, ri) > random_f64() {
                    Vec3::reflect(&unit_direction, &rec.normal)
                } else {
                    Vec3::refract(&unit_direction, &rec.normal, ri)
                };

                *scattered = Ray::new(rec.p, direction);
                true
            }
            _ => false,
        }
    }
}

fn reflectance(cosine: f64, refreaction_index: f64) -> f64 {
    // Use schlick's approximation for reflectance.
    let mut r0 = (1.0 - refreaction_index) / (1.0 + refreaction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0)
}
