use macroquad::prelude::*;

use crate::{square::Square, state::State};

const DEFAULT_COLOR: Color = BLACK;
const COLISION_COLOR: Color = PINK;

pub struct Simulation {
    state: State,
    to_screen_coords: Affine2,
}

impl Simulation {
    fn new(screen_height: f32) -> Self {
        Self {
            state: Default::default(),
            to_screen_coords: Simulation::screen_coords_transform(screen_height),
        }
    }

    pub async fn run() {
        let mut simulation = Simulation::new(screen_height());
        loop {
            simulation.draw();
            let fps = get_fps();
            draw_text(format!("{fps}"), 10., 20., 24., GRAY);
            simulation.state.update(get_frame_time());
            next_frame().await
        }
    }

    fn draw(&self) {
        clear_background(WHITE);
        let color = if Square::detect_collision(&self.state.sq1, &self.state.sq2) {
            COLISION_COLOR
        } else {
            DEFAULT_COLOR
        };
        self.state.sq1.draw(self.to_screen_coords, color);
        self.state.sq2.draw(self.to_screen_coords, color);
    }

    fn screen_coords_transform(screen_height: f32) -> Affine2 {
        let scale = 100.;
        Affine2::from_translation(Vec2::new(0., screen_height))
            * Affine2::from_scale(scale * Vec2::new(1., -1.))
    }
}
