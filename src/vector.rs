use std::ops::{Add, Div, Mul, Sub, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.x == other.x;
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        return Self {
            x: -self.x,
            y: -self.y,
        };
    }
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x: x, y: y };
    }

    pub fn from_angle(angle: f64) -> Self {
        let angle_radians = angle * std::f64::consts::PI / 180.0;
        return Vector::new(angle_radians.sin(), angle_radians.cos());
    }

    pub fn degrees(&self) -> f64 {
        return f64::atan2(self.x, self.y) * 180.0 / std::f64::consts::PI;
    }

    pub fn sqrt_magnitude(&self) -> f64 {
        return f64::sqrt(self.x.powi(2) + self.y.powi(2));
    }

    pub fn normalize(&self) -> Vector {
        return Vector::new(
            self.x / self.sqrt_magnitude(),
            self.y / self.sqrt_magnitude(),
        );
    }

    pub fn angle_to(&self, other: Vector) -> f64{
        let delta_x = self.x - other.x;
        let delta_y = other.y - self.y;

        let theta_radians = f64::atan2(delta_y, delta_x);
    
        return theta_radians * (180.0 / std::f64::consts::PI);
    }

    pub fn multiply_float(&self, val: f64) -> Vector {
        return Vector::new(self.x * val, self.y * val);
    }
}
