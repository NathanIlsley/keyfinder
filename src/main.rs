use macroquad::{miniquad::conf::Platform, prelude::*};

fn window_conf() -> Conf{
    Conf {
        window_title: String::from("Key Finder"),
        window_width: 640,
        window_height: 400,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Platform::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKBLUE);

        if is_key_down(KeyCode::D) {
            x += 5.0;
        }
        if is_key_down(KeyCode::A) {
            x -= 5.0;
        }
        if is_key_down(KeyCode::W) {
            y -= 5.0;
        }
        if is_key_down(KeyCode::S) {
            y += 5.0;
        }

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await;
    }
}