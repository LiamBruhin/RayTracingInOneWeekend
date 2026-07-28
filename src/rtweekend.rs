use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    const RAND_MAX: i32 = i32::MAX;
    rand::thread_rng().gen_range(0..RAND_MAX) as f64 / (RAND_MAX as f64 + 1.0)
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max-min)*random_f64()
}
