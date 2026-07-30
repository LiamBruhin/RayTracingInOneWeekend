use crate::rtweekend::*;

pub struct Interval {
    pub min: f64, 
    pub max: f64,
}

impl Interval {
    #[allow(dead_code)]
    pub fn empty() -> Interval {
        Interval { min: INFINITY, max: -INFINITY }
    }

    #[allow(dead_code)]
    pub fn universe() -> Interval {
        Interval { min: -INFINITY, max: INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min }  
        else if x > self.max { self.max }  
        else { x }
    }
}
