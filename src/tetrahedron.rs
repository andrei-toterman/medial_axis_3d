use super::point::Point;
use raylib::{
    color::Color,
    drawing::{RaylibDraw3D, RaylibDrawHandle, RaylibMode3D},
    math::Vector3,
};

#[derive(Copy, Clone)]
pub struct Tetrahedron {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,
    pub circumcenter: Point,
    pub circumradius: f64,
}

impl Tetrahedron {
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        let alpha = nalgebra::Matrix4::new(
            p1.x, p1.y, p1.z, 1.0, //
            p2.x, p2.y, p2.z, 1.0, //
            p3.x, p3.y, p3.z, 1.0, //
            p4.x, p4.y, p4.z, 1.0, //
        )
        .determinant();

        let dx = nalgebra::Matrix4::new(
            p1.x * p1.x + p1.y * p1.y + p1.z * p1.z,
            p1.y,
            p1.z,
            1.0,
            p2.x * p2.x + p2.y * p2.y + p2.z * p2.z,
            p2.y,
            p2.z,
            1.0,
            p3.x * p3.x + p3.y * p3.y + p3.z * p3.z,
            p3.y,
            p3.z,
            1.0,
            p4.x * p4.x + p4.y * p4.y + p4.z * p4.z,
            p4.y,
            p4.z,
            1.0,
        )
        .determinant();

        let dy = -nalgebra::Matrix4::new(
            p1.x * p1.x + p1.y * p1.y + p1.z * p1.z,
            p1.x,
            p1.z,
            1.0,
            p2.x * p2.x + p2.y * p2.y + p2.z * p2.z,
            p2.x,
            p2.z,
            1.0,
            p3.x * p3.x + p3.y * p3.y + p3.z * p3.z,
            p3.x,
            p3.z,
            1.0,
            p4.x * p4.x + p4.y * p4.y + p4.z * p4.z,
            p4.x,
            p4.z,
            1.0,
        )
        .determinant();

        let dz = nalgebra::Matrix4::new(
            p1.x * p1.x + p1.y * p1.y + p1.z * p1.z,
            p1.x,
            p1.y,
            1.0,
            p2.x * p2.x + p2.y * p2.y + p2.z * p2.z,
            p2.x,
            p2.y,
            1.0,
            p3.x * p3.x + p3.y * p3.y + p3.z * p3.z,
            p3.x,
            p3.y,
            1.0,
            p4.x * p4.x + p4.y * p4.y + p4.z * p4.z,
            p4.x,
            p4.y,
            1.0,
        )
        .determinant();

        let circumcenter = Point::new(dx / (2.0 * alpha), dy / (2.0 * alpha), dz / (2.0 * alpha));
        let circumradius = p1.dist(&circumcenter);

        Self {
            p1,
            p2,
            p3,
            p4,
            circumcenter,
            circumradius,
        }
    }

    pub fn has_point(&self, point: &Point) -> bool {
        (self.p1 == *point) || (self.p2 == *point) || (self.p3 == *point) || (self.p4 == *point)
    }

    pub fn has_point_circumcircle(&self, point: &Point) -> bool {
        point.dist(&self.circumcenter) <= self.circumradius
    }

    pub fn centroid(&self) -> Point {
        Point::new(
            (self.p1.x + self.p2.x + self.p3.x + self.p4.x) / 4.0,
            (self.p1.y + self.p2.y + self.p3.y + self.p4.y) / 4.0,
            (self.p1.z + self.p2.z + self.p3.z + self.p4.z) / 4.0,
        )
    }

    pub fn draw(
        &self,
        draw_handle: &mut RaylibMode3D<RaylibDrawHandle>,
        color: Color,
        outline: bool,
    ) {
        let fade = 0.3;
        if outline {
            draw_handle.draw_line_3D(Vector3::from(self.p1), Vector3::from(self.p2), color);
            draw_handle.draw_line_3D(Vector3::from(self.p2), Vector3::from(self.p3), color);
            draw_handle.draw_line_3D(Vector3::from(self.p3), Vector3::from(self.p1), color);
            draw_handle.draw_line_3D(Vector3::from(self.p4), Vector3::from(self.p1), color);
            draw_handle.draw_line_3D(Vector3::from(self.p4), Vector3::from(self.p2), color);
            draw_handle.draw_line_3D(Vector3::from(self.p4), Vector3::from(self.p3), color);
        }
        draw_handle.draw_triangle3D(
            Vector3::from(self.p1),
            Vector3::from(self.p3),
            Vector3::from(self.p2),
            color.fade(fade),
        );
        draw_handle.draw_triangle3D(
            Vector3::from(self.p1),
            Vector3::from(self.p2),
            Vector3::from(self.p4),
            color.fade(fade),
        );
        draw_handle.draw_triangle3D(
            Vector3::from(self.p3),
            Vector3::from(self.p1),
            Vector3::from(self.p4),
            color.fade(fade),
        );
        draw_handle.draw_triangle3D(
            Vector3::from(self.p4),
            Vector3::from(self.p2),
            Vector3::from(self.p3),
            color.fade(fade),
        );
    }
}
