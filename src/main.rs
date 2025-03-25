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
    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_RADIUS: f32 = 16.0;
    
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKBLUE);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::D) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::A) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::W) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::S) {
            y += MOVEMENT_SPEED * delta_time;
        }

        x = clamp(x, CIRCLE_RADIUS, screen_width() - CIRCLE_RADIUS);
        y = clamp(y, CIRCLE_RADIUS, screen_height() - CIRCLE_RADIUS);

        draw_circle(x, y, CIRCLE_RADIUS, YELLOW);

        next_frame().await;
    }
}