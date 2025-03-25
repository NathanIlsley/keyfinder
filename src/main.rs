use macroquad::prelude::*;

#[macroquad::main("Key Finder")]
async fn main() {
    loop {
        clear_background(DARKBLUE);
        next_frame().await;
    }
}