use super::point::Point;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Face {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Face {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self { p1, p2, p3 }
    }
}

impl PartialEq for Face {
    fn eq(&self, &Self { p1, p2, p3 }: &Self) -> bool {
        (self.p1 == p1 && self.p2 == p2 && self.p3 == p3)
            || (self.p1 == p1 && self.p2 == p3 && self.p3 == p2)
            || (self.p1 == p2 && self.p2 == p1 && self.p3 == p3)
            || (self.p1 == p2 && self.p2 == p3 && self.p3 == p1)
            || (self.p1 == p3 && self.p2 == p1 && self.p3 == p2)
            || (self.p1 == p3 && self.p2 == p2 && self.p3 == p1)
    }
}

impl Eq for Face {}

impl Hash for Face {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut points = [
            (
                self.p1.x.to_bits(),
                self.p1.y.to_bits(),
                self.p1.z.to_bits(),
            ),
            (
                self.p2.x.to_bits(),
                self.p2.y.to_bits(),
                self.p2.z.to_bits(),
            ),
            (
                self.p3.x.to_bits(),
                self.p3.y.to_bits(),
                self.p3.z.to_bits(),
            ),
        ];
        points.sort_unstable();
        points.hash(state);
    }
}
