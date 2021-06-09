use super::almost_equal;
use raylib::math::Vector3;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dist(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_normal(&self) -> bool {
        self.x.is_normal() && self.y.is_normal() && self.z.is_normal()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        almost_equal(self.x, other.x)
            && almost_equal(self.y, other.y)
            && almost_equal(self.z, other.z)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x.to_bits(), self.y.to_bits(), self.z.to_bits()).hash(state);
    }
}

impl From<Point> for Vector3 {
    fn from(Point { x, y, z }: Point) -> Self {
        Self::new(x as f32, y as f32, z as f32)
    }
}

impl From<Vector3> for Point {
    fn from(Vector3 { x, y, z }: Vector3) -> Self {
        Self::new(x as f64, y as f64, z as f64)
    }
}
