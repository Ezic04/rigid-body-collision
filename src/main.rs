use crate::state::State;
use macroquad::prelude::*;

mod square;
mod state;

fn window_coords_transform(window_height: f32) -> Affine2 {
    let scale = 100.;
    Affine2::from_translation(Vec2::new(0., window_height))
        * Affine2::from_scale(scale * Vec2::new(1., -1.))
}

#[macroquad::main("Colisions")]
async fn main() {
    let mut state = State::default();
    let height = screen_height();
    let to_window_coords = window_coords_transform(height);
    loop {
        state.draw(to_window_coords);
        let fps = get_fps();
        draw_text(format!("{fps}"), 10., 20., 24., GRAY);
        state.update();
        next_frame().await
    }
}
