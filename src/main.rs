use medial_axis_3d::{delaunay, face::Face, medial_axis, point::Point, point_inside_shape};
use raylib::prelude::{
    consts::CameraMode, get_random_value, rstr, Camera3D, Color, KeyboardKey, RaylibDraw,
    RaylibDraw3D, RaylibDrawGui, RaylibMode3DExt, Rectangle, Vector3,
};
use std::{
    io::{BufRead, BufReader},
    time::UNIX_EPOCH,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let file_name = args.next().expect("no file given");
    let input = BufReader::new(std::fs::File::open(&file_name).unwrap());

    let mut points = Vec::new();
    let mut faces = Vec::new();

    for line in input.lines().map(Result::unwrap) {
        let mut tokens = line.trim().split_whitespace();
        match tokens.next().unwrap() {
            "v" => {
                let (x, y, z) = (
                    tokens.next().unwrap().parse().unwrap(),
                    tokens.next().unwrap().parse().unwrap(),
                    tokens.next().unwrap().parse().unwrap(),
                );
                points.push(Point::new(x, y, z));
            }
            "f" => {
                let (p1, p2, p3) = (
                    tokens.next().unwrap().parse::<usize>().unwrap(),
                    tokens.next().unwrap().parse::<usize>().unwrap(),
                    tokens.next().unwrap().parse::<usize>().unwrap(),
                );
                faces.push(Face::new(points[p1 - 1], points[p2 - 1], points[p3 - 1]));
            }
            _ => continue,
        };
    }

    let start = std::time::Instant::now();
    let mut tetrahedrons = delaunay(&points);
    println!("{}", start.elapsed().as_micros());

    tetrahedrons.retain(|tetra| point_inside_shape(&tetra.centroid(), &faces));
    let medial_axis = medial_axis(&tetrahedrons);

    let tetrahedrons_draw = tetrahedrons
        .iter()
        .map(|t| {
            (
                t,
                Color::color_from_hsv(get_random_value::<i32>(0, 360) as f32, 0.5, 0.8),
            )
        })
        .collect::<Vec<_>>();

    let medial_axis_draw = medial_axis
        .iter()
        .map(|edge| (Vector3::from(edge.p1), Vector3::from(edge.p2)))
        .collect::<Vec<_>>();

    let (mut rl_handle, rl_thread) = raylib::init().size(1000, 1000).title("skeleton 3d").build();
    rl_handle.set_target_fps(60);

    let mut camera = Camera3D::perspective(
        Vector3::new(50.0, 50.0, 50.0),
        Vector3::zero(),
        Vector3::up(),
        45.0,
    );
    rl_handle.set_camera_mode(camera, CameraMode::CAMERA_FREE);

    let mut show_ui = true;

    let mut tetra_iter = (0..tetrahedrons.len()).cycle();
    let mut tetra_index = tetra_iter.next().unwrap();

    let mut show_delaunay = false;
    let mut show_skeleton = false;
    let mut show_skeleton_balls = false;
    let mut show_outline = true;
    let mut show_vertices = true;
    let mut show_spheres = false;
    let mut show_grid = true;

    while !rl_handle.window_should_close() {
        if rl_handle.is_key_pressed(KeyboardKey::KEY_U) {
            show_ui = !show_ui;
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_S) {
            rl_handle.take_screenshot(
                &rl_thread,
                format!(
                    "{}.png",
                    std::time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )
                .as_str(),
            );
        }

        rl_handle.update_camera(&mut camera);

        let mut draw_handle = rl_handle.begin_drawing(&rl_thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut draw_handle = draw_handle.begin_mode3D(camera);

            if show_delaunay {
                for &(tetra, color) in tetrahedrons_draw.iter() {
                    tetra.draw(&mut draw_handle, color, false);
                }
            }

            if show_skeleton {
                for (v1, v2) in medial_axis_draw.iter() {
                    draw_handle.draw_line_3D(v1, v2, Color::PURPLE);
                    if show_skeleton_balls {
                        draw_handle.draw_sphere_ex(v1, 0.5, 4, 6, Color::PURPLE);
                        draw_handle.draw_sphere_ex(v2, 0.5, 4, 6, Color::PURPLE);
                    }
                }
            }

            if show_outline {
                for face in faces.iter() {
                    draw_handle.draw_line_3D(
                        Vector3::from(face.p1),
                        Vector3::from(face.p2),
                        Color::BLACK,
                    );
                    draw_handle.draw_line_3D(
                        Vector3::from(face.p2),
                        Vector3::from(face.p3),
                        Color::BLACK,
                    );
                    draw_handle.draw_line_3D(
                        Vector3::from(face.p3),
                        Vector3::from(face.p1),
                        Color::BLACK,
                    );
                }
            }

            if show_vertices {
                for point in points.iter() {
                    draw_handle.draw_sphere_ex(
                        Vector3::from(*point),
                        0.3,
                        4,
                        6,
                        if !show_spheres {
                            Color::BLACK
                        } else {
                            let tetra = &tetrahedrons[tetra_index];
                            if tetra.has_point(point) {
                                Color::BLUE
                            } else if tetra.has_point_circumcircle(point) {
                                Color::RED
                            } else {
                                Color::GREEN
                            }
                        },
                    );
                }
            }

            if show_spheres {
                let tetra = &tetrahedrons[tetra_index];
                tetra.draw(&mut draw_handle, Color::BLUE, true);
                draw_handle.draw_sphere(
                    Vector3::from(tetra.circumcenter),
                    tetra.circumradius.sqrt() as f32,
                    Color::BLUE.fade(0.3),
                );
            }

            if show_grid {
                draw_handle.draw_grid(100, 10.0);
            }
        }

        if show_ui {
            let mut gui_y = (10..).step_by(35).map(|n| n as f32);

            show_outline = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show outline")),
                show_outline,
            );
            show_vertices = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show vertices")),
                show_vertices,
            );
            show_delaunay = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show delaunay")),
                show_delaunay,
            );
            show_skeleton = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show skeleton")),
                show_skeleton,
            );
            show_skeleton_balls = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show skeleton balls")),
                show_skeleton_balls,
            );
            show_spheres = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show spheres")),
                show_spheres,
            );
            if draw_handle.gui_button(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("{}", tetra_index).as_c_str()),
            ) {
                tetra_index = tetra_iter.next().unwrap();
            }
            show_grid = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show grid")),
                show_grid,
            );
        }
    }
}
