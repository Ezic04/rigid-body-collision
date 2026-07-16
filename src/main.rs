use macroquad::math;
use macroquad::prelude::*;
use std::f32::consts;

struct Square {
    center: Vec2,
    rot: f32,
    r: f32,
}

struct State {
    sq: Square,
    t: f32,
}

impl State {
    fn update(&mut self) {
        let dt = 0.001;
        let two_pi = 2.0 * consts::PI;
        self.t += dt;
        if self.t > two_pi {
            self.t -= two_pi
        }
        self.sq.center.x += 10.0 * self.t.sin();
    }
}

struct Scene {
    scale: f32,
}

impl Default for Scene {
    fn default() -> Self {
        Scene { scale: 0.01 }
    }
}

#[macroquad::main("Colisions")]
async fn main() {
    let mut t = 0.0;
    let dt = 0.001;
    loop {
        let two_pi = 2.0 * consts::PI;
        t += dt;
        if t > two_pi {
            t -= two_pi
        }
        let h = screen_height();
        let w = screen_width();
        clear_background(WHITE);
        draw_poly(w / 2.0 + w / 4.0 * t.sin(), h / 2.0, 4, 128.0, 0.0, BLACK);
        next_frame().await
    }
}
