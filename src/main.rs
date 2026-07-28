use itertools::{Itertools, MinMaxResult};
use macroquad::prelude::*;
use std::f32::consts;
use angle::{Angle, Rad};

#[derive(Debug)]
struct Square {
    position: Vec2,
    linear_velocity: Vec2,
    rotation: Rad<f32>,
    angular_velocity: Rad<f32>,
    radius: f32,
    mass: f32,
}

impl Square {
    fn draw(&self, to_window_cords: Affine2) {
        let window_center = to_window_cords.transform_point2(self.position);
        let window_radius = to_window_cords
            .transform_vector2(Vec2::new(self.radius, 0.))
            .x;
        draw_poly(
            window_center.x,
            window_center.y,
            4,
            window_radius,
            self.rotation.to_deg().value(),
            BLACK,
        );
    }

    fn get_normals(&self) -> Vec<Vec2> {
        let v = Vec2::from_angle(self.rotation.value());
        vec![v, v.perp()]
    }

    fn get_vertices(&self) -> Vec<Vec2> {
        (0..4)
            .map(|i| self.position + self.radius * Vec2::from_angle(consts::FRAC_PI_2 * i as f32))
            .collect::<Vec<_>>()
    }
}

impl Default for Square {
    fn default() -> Self {
        let r = 0.5;
        let density = 10.;
        Self {
            position: Vec2::new(2., 2.),
            linear_velocity: Vec2::new(1., 0.5),
            rotation: Rad(0.),
            angular_velocity: Rad::two_pi() * 0.1,
            radius: r,
            mass: density * r,
        }
    }
}

#[derive(Debug)]
struct State {
    square: Square,
    time: f32,
}

impl State {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;
        if self.time > consts::TAU {
            self.time -= consts::TAU
        }
        let sq = &mut self.square;
        sq.position += sq.linear_velocity * dt;
        sq.rotation += sq.angular_velocity * dt;
    }

    fn draw(&self, to_window_cords: Affine2) {
        clear_background(WHITE);
        self.square.draw(to_window_cords)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            square: Square::default(),
            time: 0.,
        }
    }
}

fn window_coords_transform(window_height: f32) -> Affine2 {
    let scale = 100.;
    Affine2::from_translation(Vec2::new(0., window_height))
        * Affine2::from_scale(scale * Vec2::new(1., -1.))
}

fn check_proj_overlap(normal: &Vec2, sq1: &Square, sq2: &Square) -> bool {
    // let is_steep = f32::abs(normal.x / normal.y);
    let is_steep = f32::abs(normal.x) <= f32::abs(normal.y);

    let calc_proj = |verts: Vec<Vec2>| {
        let MinMaxResult::MinMax(min, max) = verts.iter().map(|v| {
            let proj = v.project_onto(normal.clone());
            if is_steep { proj.y } else { proj.x }
        }).minmax() else {
            panic!("MinMax failed...")
        };

        (min, max)
    };

    let verts1 = sq1.get_vertices();
    let verts2 = sq2.get_vertices();

    let (min1, max1) = calc_proj(verts1);
    let (min2, max2) = calc_proj(verts2);

    max1 <= min2 || max2 <= min1
}

fn detect_collision(sq1: &Square, sq2: &Square) -> bool {
    let normals1 = sq1.get_normals();
    let normals2 = sq2.get_normals();

    normals1.iter().chain(normals2.iter())
        .any(|n| check_proj_overlap(n, sq1, sq2))
}

#[macroquad::main("Colisions")]
async fn main() {
    let mut state = State::default();
    loop {
        let height = screen_height();
        let to_window_cords = window_coords_transform(height);
        state.update();
        state.draw(to_window_cords);
        next_frame().await
    }
}
