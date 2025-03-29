use core::num;
use std::{thread::sleep, time::Duration};

use macroquad::{miniquad::conf::Platform, prelude::*};

fn window_conf() -> Conf{
    Conf {
        window_title: String::from("Key Finder"),
        window_width: 800,
        window_height: 500,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Platform::default(),
    }
}

#[derive(Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn norm(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }

    fn mul(&self, val: f32) -> Vector2 {
        Vector2 { x: self.x * val, y: self.y * val }
    }

    fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn inc(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    fn dec(&mut self, x: f32, y: f32) {
        self.x -= x;
        self.y -= y;
    }

    fn clamp(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Vector2 {
        let newx = clamp(self.x, xmin, xmax);
        let newy = clamp(self.y, ymin, ymax);

        let changes = Vector2 { x: self.x - newx, y: self.y - newy };
        
        self.x = newx;
        self.y = newy;

        changes 
    }
}

struct Player {
    position: Vector2,
    velocity: Vector2,
    // platforms: Vec<Plat>,
    grounded: bool,
    jump_time: f32,

    scroll: f32,
}

impl Player {
    const GRAVITY: f32 = 4000.0;
    const X_MOVEMENT_SPEED: f32 = 600.0;
    const X_RESPONSIVENESS: f32 = 5000.0;
    const X_RESPONSIVENESS_AIR: f32 = 1000.0;
    const JUMP_SPEED: f32 = 700.0;
    
    const CIRCLE_RADIUS: f32 = 16.0;

    fn new() -> Self {
        Self {
            position: Vector2 { x: screen_width() / 2.0, y: screen_height() * 0.75 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            // platforms: vec![],
            grounded: false,
            jump_time: 0.0,

            scroll: 0.0,
        }
    }

    // fn set_plats(&mut self, plats: Vec<Plat>) {
    //     self.platforms = plats;
    // }
    
    fn update(&mut self, platforms: &[Plat]) {
        let delta_time = get_frame_time();

        // Inputs
        if self.grounded {
            self.velocity.x += match (is_key_down(KeyCode::D), is_key_down(KeyCode::A)) {
                (true, false) => Player::X_RESPONSIVENESS * delta_time,
                (false, true) => -Player::X_RESPONSIVENESS * delta_time,
                _             => Player::X_RESPONSIVENESS * if self.velocity.x.abs() > Player::X_RESPONSIVENESS * delta_time {-self.velocity.x / self.velocity.x.abs()} else {self.velocity.x = 0.0; 0.0} * delta_time,
            };
        } else {
            self.velocity.x += match (is_key_down(KeyCode::D), is_key_down(KeyCode::A)) {
                (true, false) => Player::X_RESPONSIVENESS_AIR * delta_time,
                (false, true) => -Player::X_RESPONSIVENESS_AIR * delta_time,
                _             => Player::X_RESPONSIVENESS * if self.velocity.x.abs() > Player::X_RESPONSIVENESS * delta_time {-self.velocity.x / self.velocity.x.abs()} else {self.velocity.x = 0.0; 0.0} * delta_time,
            };
        }

        self.velocity.x = clamp(self.velocity.x, -Player::X_MOVEMENT_SPEED, Player::X_MOVEMENT_SPEED);

        if is_key_down(KeyCode::W) {
            if self.grounded {
                // Apply jump velocity
                self.velocity.y = -Player::JUMP_SPEED;
                self.jump_time = 0.3;
            } else if self.jump_time != 0.0 {
                self.jump_time -= delta_time;
            }
        } else {
            self.jump_time = 0.0;
        }

        if self.jump_time <= 0.0 {
            // Apply gravity
            self.velocity.y += Player::GRAVITY * (delta_time + self.jump_time.abs());
            self.jump_time = 0.0;
        }

        // Update Position
        self.position = self.position.add(&self.velocity.mul(delta_time));

        // Check Collisions
        self.grounded = false;

        if self.position.clamp(Player::CIRCLE_RADIUS, screen_width() - Player::CIRCLE_RADIUS, -f32::INFINITY, screen_height() - Player::CIRCLE_RADIUS).y != 0.0 {
            self.velocity.y = 0.0;
            
            if self.position.y == screen_height() - Player::CIRCLE_RADIUS {
                self.grounded = true;
            }
        }

        let mut dist: Vector2;
        let mut prev_dist: Vector2;
        let mut overlap: Vector2;
        let mut in_x: bool;
        let mut in_y: bool;

        for plat in platforms {
            dist = self.position.sub(&plat.position);
            prev_dist = self.position.sub(&(plat.position.sub(&self.velocity.mul(delta_time))));

            in_x = dist.x.abs() < plat.dimensions.x / 2.0 + Player::CIRCLE_RADIUS;
            in_y = dist.y.abs() < plat.dimensions.y / 2.0 + Player::CIRCLE_RADIUS;

            if (in_x && in_y) || (prev_dist.x * dist.x < 0.0 && in_y || prev_dist.y * dist.y < 0.0 && in_x || prev_dist.x * dist.x < 0.0 && prev_dist.y * dist.y < 0.0) {
                overlap = Vector2::new(
                    if dist.x > 0.0 {plat.position.x + plat.dimensions.x / 2.0 - (self.position.x - Player::CIRCLE_RADIUS)} else {self.position.x + Player::CIRCLE_RADIUS - (plat.position.x - plat.dimensions.x / 2.0)},
                    if dist.y > 0.0 {plat.position.y + plat.dimensions.y / 2.0 - (self.position.y - Player::CIRCLE_RADIUS)} else {self.position.y + Player::CIRCLE_RADIUS - (plat.position.y - plat.dimensions.y / 2.0)},
                );

                if overlap.y < overlap.x {
                    self.position.y = if dist.y < 0.0 {self.grounded = true; plat.position.y - plat.dimensions.y / 2.0 - Player::CIRCLE_RADIUS} else {plat.position.y + plat.dimensions.y / 2.0 + Player::CIRCLE_RADIUS};
                    self.velocity.y = 0.0;
                } else {
                    self.position.x = if dist.x < 0.0 {plat.position.x - plat.dimensions.x / 2.0 - Player::CIRCLE_RADIUS} else {plat.position.x + plat.dimensions.x / 2.0 + Player::CIRCLE_RADIUS};
                    self.velocity.x = 0.0;
                }
            }
        }

        // Draw on screen
        draw_circle(self.position.x, self.position.y + self.scroll, Player::CIRCLE_RADIUS, YELLOW);

    }

    fn set_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
    }

    fn adjusted_y(&self) -> f32 {
        self.position.y + self.scroll
    }
}

#[derive(Clone, Copy)]
struct Plat {
    position: Vector2,
    dimensions: Vector2,

    scroll: f32,
}

impl Plat {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { position: Vector2::new(x, y) , dimensions: Vector2::new(w, h), scroll: 0.0 }
    }

    fn update(&mut self) {
        // Draw on screen
        let adjusted_y = self.position.y + self.scroll; // Adjust y for scroll

        if adjusted_y > -self.dimensions.y && adjusted_y < screen_height() {
            draw_rectangle(self.position.x - self.dimensions.x / 2.0, adjusted_y - self.dimensions.y / 2.0, self.dimensions.x, self.dimensions.y, WHITE);
        }
    }
    
    fn set_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
    }
}

struct Screen {
    // player: Player,
    // platforms: Vec<Plat>,
    scroll_pos: f32,
}

impl Screen {

    fn new() -> Self {
        // Screen { player: player, platforms: vec![], scroll_pos: 0.0 }
        Screen { scroll_pos: 0.0 }
    }

    // fn set_plats(&mut self, plats: Vec<Plat>) {
    //     self.platforms = plats;
    // }

    fn update(&mut self, player: &mut Player, platforms: &mut [Plat]) {

        self.scroll_pos += 0.1 * if self.scroll_pos <= 0.0 && player.adjusted_y() > screen_height() * 0.6 {0.0} else {screen_height() * 0.6 - player.adjusted_y()};

        player.set_scroll(self.scroll_pos);
        for plat in platforms {
            plat.set_scroll(self.scroll_pos);
        }
    }
}

fn generate_platforms() -> Vec<Plat> {
    let num_of_platforms = 20;
    let mut platforms = vec![];

    for i in 0..num_of_platforms {
        platforms.append(&mut vec![Plat::new(100.0 + (i % 2) as f32 * 300.0, screen_height() - i as f32 * 200.0, 300.0, 25.0)]);
    }

    platforms
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut platforms = generate_platforms();
    let mut screen = Screen::new();

    // player.set_plats(platforms);
    // screen.set_plats(platforms);    

    loop {
        clear_background(DARKBLUE);
        
        player.update(&platforms);

        for platform in &mut platforms {
            platform.update();
        }

        screen.update(&mut player, &mut platforms);

        // println!("{}", get_fps());
        
        frame_control();
        next_frame().await;
    }
}

#[cfg(target_os="windows")]
fn frame_control() {
    sleep(Duration::from_millis(1));
}

#[cfg(not(target_os="windows"))]
fn frame_control() {}