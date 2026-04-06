use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    loop {
        clear_background(GRAY);

        next_frame().await;
    }
}
