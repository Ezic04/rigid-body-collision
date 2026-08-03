use crate::square::Square;
use angle::Rad;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct State {
    pub sq1: Square,
    pub sq2: Square,
}

impl State {
    pub fn update(&mut self, dt: f32) {
        let update_sqare = |sq: &mut Square| {
            sq.position += sq.linear_velocity * dt;
            sq.rotation += sq.angular_velocity * dt;
        };
        update_sqare(&mut self.sq1);
        update_sqare(&mut self.sq2);
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
