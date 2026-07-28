use std::ops;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 { 
        Vec3 { e: [0.0, 0.0, 0.0] } 
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 { 
        Vec3 { e: [e0, e1, e2] } 
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1]  + self.e[2]*self.e[2] 
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        return self.e[0] * other.e[0]
             + self.e[1] * other.e[1]
             + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[1] * other.e[2] - self.e[2] * other.e[1],
                  self.e[2] * other.e[0] - self.e[0] * other.e[2],
                  self.e[0] * other.e[1] - self.e[1] * other.e[0])
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0/t;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Vec3 {
        Vec3::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Vec3 {
        Vec3::new(self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2])
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0]*t, self.e[1]*t, self.e[2]*t)
    }
}

// killing myself because i HAVE to do this twice changing only one thing
// Rust sucks balls
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.e[0]*self, v.e[1]*self, v.e[2]*self)
    }
}

impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.e[0]*(self as f64), v.e[1]*(self as f64), v.e[2]*(self as f64))
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0/t)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
