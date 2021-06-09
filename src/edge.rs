use super::point::Point;

pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}
