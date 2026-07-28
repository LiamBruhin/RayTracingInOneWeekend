use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() {
    const RAND_MAX: i32 = i32::MAX;
    rand::thread_rng().gen_range(0..RAND_MAX) / (RAND_MAX as f64 + 1.0)
}
