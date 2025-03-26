use std::f32::INFINITY;
use macroquad::{miniquad::conf::Platform, prelude::*};

fn window_conf() -> Conf{
    Conf {
        window_title: String::from("Key Finder"),
        window_width: 1600,
        window_height: 1000,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Platform::default(),
    }
}

struct Sprite {
    position: [f32; 2],
    velocity: [f32; 2],
}

#[macroquad::main(window_conf)]
async fn main() {
    const X_MOVEMENT_SPEED: f32 = 400.0;
    const Y_MOVEMENT_SPEED: f32 = 1600.0;
    const JUMP_SPEED: f32 = 1000.0;
    const GRAVITY: f32 = 2000.0;
    const CIRCLE_RADIUS: f32 = 16.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut ydot = 0.0;

    loop {
        clear_background(DARKBLUE);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::D) {
            x += X_MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::A) {
            x -= X_MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::W) && y == screen_height() - CIRCLE_RADIUS {
            ydot = -JUMP_SPEED;
        }

        ydot += GRAVITY * delta_time;
        ydot = clamp(ydot, -Y_MOVEMENT_SPEED, INFINITY);

        y += ydot * delta_time;

        x = clamp(x, CIRCLE_RADIUS, screen_width() - CIRCLE_RADIUS);
        let yclamp = clamp(y, CIRCLE_RADIUS, screen_height() - CIRCLE_RADIUS);

        if yclamp != y {
            ydot = 0.0;
        }
        y = yclamp;

        // set_camera(&Camera2D {
        //     zoom: vec2(1.0 / 1600.0, 1.0 / 1000.0), // Normalizes the coordinate system
        //     target: vec2(1600.0 / 2.0, 1000.0 / 2.0),
        //     ..Default::default()
        // });

        draw_circle(x, y, CIRCLE_RADIUS, YELLOW);

        next_frame().await;
    }
}