use serde::{Deserialize, Serialize};
use std::ops;

use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn reflect(a: Vec3, b: Vec3) -> Vec3 {
        a - b * Vec3::dot(a, b) * 2.0
    }

    pub fn refract(a: Vec3, b: Vec3, ni_over_nt: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(a * -1.0, b), 1.0);
        let perpendicular = (a + b * cos_theta) * ni_over_nt;
        let parallel = b * -(1.0 - Vec3::dot(perpendicular, perpendicular)).abs().sqrt();
        parallel + perpendicular
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut v = Vec3::new(5.5, 5.5, 5.5);
        let vec1 = Vec3::new(1.0, 1.0, 1.0);

        let mut rng = thread_rng();
        while v.length() >= 1.0 {
            v = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - vec1;
        }

        v
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn length(self) -> f64 {
        Vec3::dot(self, self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / _rhs.x, self.y / _rhs.y, self.z / _rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}
