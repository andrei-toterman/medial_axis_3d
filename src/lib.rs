pub mod edge;
pub mod face;
pub mod point;
pub mod tetrahedron;

use edge::Edge;
use face::Face;
use nalgebra::Vector3;
use point::Point;
use std::collections::{HashMap, HashSet};
use tetrahedron::Tetrahedron;

pub fn almost_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}

pub fn delaunay(points: &[Point]) -> Vec<Tetrahedron> {
    let mut tetrahedrons = Vec::new();

    let Point {
        x: mut min_x,
        y: mut min_y,
        z: mut min_z,
    } = points[0];
    let (mut max_x, mut max_y, mut max_z) = (min_x, min_y, min_z);

    for &Point { x, y, z } in points.iter() {
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if z < min_z {
            min_z = z;
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if z > max_z {
            max_z = z;
        }
    }

    let d_max = *[max_x - min_x, max_y - min_y, max_z - min_z]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let (mid_x, mid_z) = ((min_x + max_x) / 2.0, (min_z + max_z) / 2.0);

    let super_p1 = Point::new(min_x - 20.0 * d_max, min_y - d_max, min_z - 20.0 * d_max);
    let super_p2 = Point::new(max_x + 20.0 * d_max, min_y - d_max, min_z - 20.0 * d_max);
    let super_p3 = Point::new(mid_x, min_y - d_max, max_z + 20.0 * d_max);
    let super_p4 = Point::new(mid_x, max_y + 20.0 * d_max, mid_z);

    tetrahedrons.push((
        Tetrahedron::new(super_p1, super_p2, super_p3, super_p4),
        false,
    ));

    let mut hole = HashSet::new();
    for point in points.iter() {
        for (tetra, bad) in tetrahedrons.iter_mut() {
            if tetra.has_point_circumcircle(point) {
                *bad = true;
                let faces = [
                    Face::new(tetra.p1, tetra.p2, tetra.p3),
                    Face::new(tetra.p1, tetra.p2, tetra.p4),
                    Face::new(tetra.p1, tetra.p3, tetra.p4),
                    Face::new(tetra.p2, tetra.p3, tetra.p4),
                ];
                for &face in faces.iter() {
                    if !hole.insert(face) {
                        hole.remove(&face);
                    }
                }
            }
        }

        tetrahedrons.retain(|&(_, bad)| !bad);

        for face in hole.iter() {
            tetrahedrons.push((Tetrahedron::new(face.p1, face.p2, face.p3, *point), false));
        }
        tetrahedrons.retain(|(tetra, _)| tetra.circumcenter.is_normal());
        hole.clear();
    }

    tetrahedrons
        .into_iter()
        .filter_map(|(tetra, _)| {
            if [super_p1, super_p2, super_p3, super_p4]
                .iter()
                .all(|p| !tetra.has_point(p))
            {
                Some(tetra)
            } else {
                None
            }
        })
        .collect()
}

pub fn face_adjacency(
    tetrahedrons: &[Tetrahedron],
) -> HashMap<Face, (Tetrahedron, Option<Tetrahedron>)> {
    let mut faces = HashMap::new();

    for &tetra in tetrahedrons {
        faces
            .entry(Face::new(tetra.p1, tetra.p2, tetra.p3))
            .and_modify(|(_, t)| *t = Some(tetra))
            .or_insert((tetra, None));
        faces
            .entry(Face::new(tetra.p1, tetra.p2, tetra.p4))
            .and_modify(|(_, t)| *t = Some(tetra))
            .or_insert((tetra, None));
        faces
            .entry(Face::new(tetra.p2, tetra.p3, tetra.p4))
            .and_modify(|(_, t)| *t = Some(tetra))
            .or_insert((tetra, None));

        faces
            .entry(Face::new(tetra.p1, tetra.p3, tetra.p4))
            .and_modify(|(_, t)| *t = Some(tetra))
            .or_insert((tetra, None));
    }

    faces
}

pub fn medial_axis(tetrahedrons: &[Tetrahedron]) -> Vec<Edge> {
    face_adjacency(tetrahedrons)
        .into_iter()
        .filter_map(|(_, (t1, t2))| t2.map(|t2| Edge::new(t1.circumcenter, t2.circumcenter)))
        .collect()
}

pub fn point_inside_shape(point: &Point, shape: &[Face]) -> bool {
    fn orient_3d(points: [&Point; 4]) -> f64 {
        let a = Vector3::new(points[0].x, points[0].y, points[0].z);
        let b = Vector3::new(points[1].x, points[1].y, points[1].z);
        let c = Vector3::new(points[2].x, points[2].y, points[2].z);
        let d = Vector3::new(points[3].x, points[3].y, points[3].z);
        Vector3::dot(&Vector3::cross(&(b - a), &(c - a)), &(d - a)).signum()
    }

    fn intersect(
        Edge { p1: q1, p2: q2 }: &Edge,
        Face {
            p1: t1,
            p2: t2,
            p3: t3,
        }: &Face,
    ) -> bool {
        let s1 = orient_3d([q1, t1, t2, t3]);
        let s2 = orient_3d([q2, t1, t2, t3]);
        if almost_equal(s1, s2) {
            return false;
        }
        let s3 = orient_3d([q1, q2, t1, t2]);
        let s4 = orient_3d([q1, q2, t2, t3]);
        let s5 = orient_3d([q1, q2, t3, t1]);
        almost_equal(s3, s4) && almost_equal(s4, s5)
    }

    let segment = Edge::new(*point, Point::new(point.x, point.y, point.z + 1e30));
    let mut inside = false;

    for face in shape.iter() {
        if intersect(&segment, face) {
            inside = !inside;
        }
    }

    shape.is_empty() || inside
}
