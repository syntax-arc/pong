use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main() {
    loop {
        clear_background(GRAY);

        next_frame().await;
    }
}
