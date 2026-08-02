use angle::{Angle, Rad};
use itertools::Itertools;
use macroquad::prelude::*;
use std::f32::consts;

#[derive(Debug, Clone)]
pub struct Square {
    pub position: Vec2,
    pub linear_velocity: Vec2,
    pub rotation: Rad<f32>,
    pub angular_velocity: Rad<f32>,
    pub radius: f32,
    // pub mass: f32,
}

impl Square {
    // fn new(
    //     position: Vec2,
    //     linear_velocity: Vec2,
    //     rotation: Rad<f32>,
    //     angular_velocity: Rad<f32>,
    //     radius: f32,
    // ) -> Self {
    //     let density = 10.;
    //     Self {
    //         position,
    //         linear_velocity,
    //         rotation,
    //         angular_velocity,
    //         radius,
    //         mass: radius * density,
    //     }
    // }

    pub fn draw(&self, to_window_cords: Affine2, color: Color) {
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

    pub fn detect_collision(sq1: &Self, sq2: &Self) -> bool {
        let normals1 = sq1.get_normals();
        let normals2 = sq2.get_normals();
        normals1
            .iter()
            .chain(normals2.iter())
            .all(|n| Self::check_proj_overlap(n, sq1, sq2))
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
                    * Vec2::from_angle(
                        rot + consts::FRAC_PI_4 + consts::FRAC_PI_2 * i as f32,
                    )
        })
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
        // let density = 10.;
        Self {
            position: Vec2::new(2., 2.),
            linear_velocity: Vec2::new(1., 0.5),
            rotation: Rad(consts::FRAC_PI_4),
            angular_velocity: -Rad::two_pi() * 0.1,
            radius,
            // mass: density * radius,
        }
    }
}
