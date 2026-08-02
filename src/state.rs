use crate::square::Square;
use angle::Rad;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct State {
    sq1: Square,
    sq2: Square,
}

impl State {
    pub fn update(&mut self) {
        let dt = get_frame_time();
        let update_sqare = |sq: &mut Square| {
            sq.position += sq.linear_velocity * dt;
            sq.rotation += sq.angular_velocity * dt;
        };
        update_sqare(&mut self.sq1);
        update_sqare(&mut self.sq2);
    }

    pub fn draw(&self, to_window_coords: Affine2) {
        clear_background(WHITE);
        let color = if Square::detect_collision(&self.sq1, &self.sq2) {
            PINK
        } else {
            BLACK
        };
        self.sq1.draw(to_window_coords, color);
        self.sq2.draw(to_window_coords, color);
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
