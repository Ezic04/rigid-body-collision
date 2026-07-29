use angle::{Angle, Rad};
use itertools::Itertools;
use macroquad::prelude::*;
use std::f32::consts;

#[derive(Debug, Clone)]
struct Square {
    position: Vec2,
    linear_velocity: Vec2,
    rotation: Rad<f32>,
    angular_velocity: Rad<f32>,
    radius: f32,
    mass: f32,
}

impl Square {
    fn new(
        position: Vec2,
        linear_velocity: Vec2,
        rotation: Rad<f32>,
        angular_velocity: Rad<f32>,
        radius: f32,
    ) -> Self {
        let density = 10.;
        Self {
            position,
            linear_velocity,
            rotation,
            angular_velocity,
            radius,
            mass: radius * density,
        }
    }

    fn draw(&self, to_window_cords: Affine2, color: Color) {
        let window_center = to_window_cords.transform_point2(self.position);
        let window_radius = to_window_cords
            .transform_vector2(Vec2::new(self.radius, 0.))
            .x;
        draw_poly(
            window_center.x,
            window_center.y,
            4,
            window_radius,
            45. - self.rotation.to_deg().value(),
            color,
        );
    }

    fn get_normals(&self) -> [Vec2; 2] {
        let v = Vec2::from_angle(self.rotation.value());
        [v, v.perp()]
    }

    fn get_vertices(&self) -> [Vec2; 4] {
        let rot = self.rotation.value();
        std::array::from_fn(|i| {
            self.position
                + self.radius
                    * Vec2::from_angle(rot + consts::FRAC_PI_4 + consts::FRAC_PI_2 * i as f32)
        })
    }

    fn detect_collision(sq1: &Self, sq2: &Self) -> bool {
        let normals1 = sq1.get_normals();
        let normals2 = sq2.get_normals();
        normals1
            .iter()
            .chain(normals2.iter())
            .all(|n| Self::check_proj_overlap(n, sq1, sq2))
    }

    fn check_proj_overlap(normal: &Vec2, sq1: &Self, sq2: &Self) -> bool {
        let calc_proj = |verts: [Vec2; 4]| {
            verts
                .into_iter()
                .map(|v| v.dot(*normal))
                .minmax_by(f32::total_cmp)
                .into_option()
                .expect("minmax failed")
        };
        let (min1, max1) = calc_proj(sq1.get_vertices());
        let (min2, max2) = calc_proj(sq2.get_vertices());
        min1 <= max2 && min2 <= max1
    }
}

impl Default for Square {
    fn default() -> Self {
        let radius = 0.5;
        let density = 10.;
        Self {
            position: Vec2::new(2., 2.),
            linear_velocity: Vec2::new(1., 0.5),
            rotation: Rad(consts::FRAC_PI_4),
            angular_velocity: -Rad::two_pi() * 0.1,
            radius,
            mass: density * radius,
        }
    }
}

#[derive(Debug)]
struct State {
    sq1: Square,
    sq2: Square,
}

impl State {
    fn update(&mut self) {
        let dt = get_frame_time();
        let update_sqare = |sq: &mut Square| {
            sq.position += sq.linear_velocity * dt;
            sq.rotation += sq.angular_velocity * dt;
        };
        update_sqare(&mut self.sq1);
        update_sqare(&mut self.sq2);
    }

    fn draw(&self, to_window_cords: Affine2) {
        clear_background(WHITE);
        let color = if Square::detect_collision(&self.sq1, &self.sq2) {
            PINK
        } else {
            BLACK
        };
        self.sq1.draw(to_window_cords, color);
        self.sq2.draw(to_window_cords, color);
    }
}

impl Default for State {
    fn default() -> Self {
        let default = Square::default();
        Self {
            sq1: default.clone(),
            sq2: Square {
                position: default.position.with_x(4.),
                rotation: Rad(0.),
                angular_velocity: -default.angular_velocity,
                linear_velocity: default.linear_velocity.perp(),
                ..default
            },
        }
    }
}

fn window_coords_transform(window_height: f32) -> Affine2 {
    let scale = 100.;
    Affine2::from_translation(Vec2::new(0., window_height))
        * Affine2::from_scale(scale * Vec2::new(1., -1.))
}

#[macroquad::main("Colisions")]
async fn main() {
    let mut state = State::default();
    let height = screen_height();
    let to_window_cords = window_coords_transform(height);
    loop {
        state.draw(to_window_cords);
        let fps = get_fps();
        draw_text(format!("{fps}"), 10., 20., 24., GRAY);
        state.update();
        next_frame().await
    }
}
