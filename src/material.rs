use crate::color::*;
use crate::ray::*;
use crate::hits::*;
use crate::vectors::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    None,
    // Diffuse,
    Lambertian(Color),
    Metal(Color),
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
            Material::Metal(albedo) => {
                let reflected: Vec3 = Vec3::reflect(&r_in.direction(), &rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                true
            }
            _ => false,
        }
    }
}
