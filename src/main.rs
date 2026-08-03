mod simulation;
mod square;
mod state;

use crate::simulation::Simulation;

#[macroquad::main("Colisions")]
async fn main() {
    Simulation::run().await
}
