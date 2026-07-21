use macroquad::prelude::*;
use std::f32::consts;

#[derive(Debug)]
struct Square {
    position: Vec2,
    linear_velocity: Vec2,
    rotation: f32,
    angular_velocity: f32,
    radius: f32,
    mass: f32,
}

impl Square {
    fn draw(&self, to_window_cords: Affine2) {
        let window_center = to_window_cords.transform_point2(self.position);
        let window_radius = to_window_cords
            .transform_vector2(Vec2::new(self.radius, 0.0))
            .x;
        draw_poly(
            window_center.x,
            window_center.y,
            4,
            window_radius,
            f32::to_degrees(self.rotation),
            BLACK,
        );
    }
}

impl Default for Square {
    fn default() -> Self {
        let r = 0.5;
        let density = 10.0;
        Self {
            position: Vec2::new(2.0, 2.0),
            linear_velocity: Vec2::new(1.0, 0.5),
            rotation: 0.0,
            angular_velocity: 0.1 * consts::TAU,
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
            time: 0.0,
        }
    }
}

fn window_coords_transform(window_height: f32) -> Affine2 {
    let scale = 100.0;
    Affine2::from_translation(Vec2::new(0.0, window_height))
        * Affine2::from_scale(scale * Vec2::new(1.0, -1.0))
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
