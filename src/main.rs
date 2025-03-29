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
    const X_MOVEMENT_SPEED: f32 = 500.0;
    const X_RESPONSIVENESS: f32 = 3000.0;
    const X_RESPONSIVENESS_AIR: f32 = 1500.0;
    const JUMP_SPEED: f32 = 600.0;
    
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
                    self.position.y = if dist.y < 0.0 {self.grounded = true; plat.position.y - plat.dimensions.y / 2.0 - Player::CIRCLE_RADIUS} else {self.jump_time = 0.0; plat.position.y + plat.dimensions.y / 2.0 + Player::CIRCLE_RADIUS};
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

struct Enemy {
    position: Vector2,
    velocity: Vector2,
    grounded: bool,
    
    scroll: f32,
}

impl Enemy {
    const DIMENSION: f32 = 16.0;

    fn new(x: f32, y: f32, vel_x: f32) -> Self {
        Self { position: Vector2::new(x, y) , velocity: Vector2::new(vel_x, 0.0), scroll: 0.0, grounded: false }
    }

    fn update(&mut self, platforms: &[Plat], player: &Player) {
        let delta_time = get_frame_time();

        if self.grounded {
            self.velocity.y = -2000.0;
        }

        // Apply gravity
        self.velocity.y += Player::GRAVITY * delta_time;

        // Update Position
        self.position = self.position.add(&self.velocity.mul(delta_time));

        // Check Collisions
        self.grounded = false;

        let pos_clamp = self.position.clamp(Enemy::DIMENSION, screen_width() - Enemy::DIMENSION, -f32::INFINITY, screen_height() - Enemy::DIMENSION);
        if pos_clamp.y != 0.0 {
            self.velocity.y = 0.0;
            
            if self.position.y == screen_height() - Enemy::DIMENSION {
                self.grounded = true;
            }
        }

        if pos_clamp.x != 0.0 {
            self.velocity.x = -self.velocity.x
        }

        let mut dist: Vector2;
        let mut prev_dist: Vector2;
        let mut overlap: Vector2;
        let mut in_x: bool;
        let mut in_y: bool;

        for plat in platforms {
            dist = self.position.sub(&plat.position);
            prev_dist = self.position.sub(&(plat.position.sub(&self.velocity.mul(delta_time))));

            in_x = dist.x.abs() < plat.dimensions.x / 2.0 + Enemy::DIMENSION;
            in_y = dist.y.abs() < plat.dimensions.y / 2.0 + Enemy::DIMENSION;

            if (in_x && in_y) || (prev_dist.x * dist.x < 0.0 && in_y || prev_dist.y * dist.y < 0.0 && in_x || prev_dist.x * dist.x < 0.0 && prev_dist.y * dist.y < 0.0) {
                overlap = Vector2::new(
                    if dist.x > 0.0 {plat.position.x + plat.dimensions.x / 2.0 - (self.position.x - Enemy::DIMENSION)} else {self.position.x + Enemy::DIMENSION - (plat.position.x - plat.dimensions.x / 2.0)},
                    if dist.y > 0.0 {plat.position.y + plat.dimensions.y / 2.0 - (self.position.y - Enemy::DIMENSION)} else {self.position.y + Enemy::DIMENSION - (plat.position.y - plat.dimensions.y / 2.0)},
                );
                
                if overlap.y < overlap.x {
                    self.position.y = if dist.y < 0.0 {self.grounded = true; plat.position.y - plat.dimensions.y / 2.0 - Enemy::DIMENSION} else {plat.position.y + plat.dimensions.y / 2.0 + Enemy::DIMENSION};
                    self.velocity.y = 0.0;
                } else {
                    self.position.x = if dist.x < 0.0 {plat.position.x - plat.dimensions.x / 2.0 - Enemy::DIMENSION} else {plat.position.x + plat.dimensions.x / 2.0 + Enemy::DIMENSION};
                    self.velocity.x = -self.velocity.x;
                }
            }
        }

        dist = self.position.sub(&player.position);
        prev_dist = self.position.sub(&(player.position.sub(&self.velocity.mul(delta_time))));

        in_x = dist.x.abs() < Player::CIRCLE_RADIUS / 2.0 + Enemy::DIMENSION;
        in_y = dist.y.abs() < Player::CIRCLE_RADIUS / 2.0 + Enemy::DIMENSION;

        if (in_x && in_y) || (prev_dist.x * dist.x < 0.0 && in_y || prev_dist.y * dist.y < 0.0 && in_x || prev_dist.x * dist.x < 0.0 && prev_dist.y * dist.y < 0.0) {
            overlap = Vector2::new(
                if dist.x > 0.0 {player.position.x + Player::CIRCLE_RADIUS / 2.0 - (self.position.x - Enemy::DIMENSION)} else {self.position.x + Enemy::DIMENSION - (player.position.x - Player::CIRCLE_RADIUS / 2.0)},
                if dist.y > 0.0 {player.position.y + Player::CIRCLE_RADIUS / 2.0 - (self.position.y - Enemy::DIMENSION)} else {self.position.y + Enemy::DIMENSION - (player.position.y - Player::CIRCLE_RADIUS / 2.0)},
            );
            
            if overlap.y < overlap.x {
                self.position.y = if dist.y < 0.0 {self.grounded = true; player.position.y - Player::CIRCLE_RADIUS / 2.0 - Enemy::DIMENSION} else {player.position.y + Player::CIRCLE_RADIUS / 2.0 + Enemy::DIMENSION};
                self.velocity.y = 0.0;
            } else {
                self.position.x = if dist.x < 0.0 {player.position.x - Player::CIRCLE_RADIUS / 2.0 - Enemy::DIMENSION} else {player.position.x + Player::CIRCLE_RADIUS / 2.0 + Enemy::DIMENSION};
                self.velocity.x = -self.velocity.x;
            }
        }

        // Draw on screen
        let adjusted_y = self.position.y + self.scroll;
        if adjusted_y > -Enemy::DIMENSION && adjusted_y < screen_height() + Enemy::DIMENSION {
            draw_circle(self.position.x, adjusted_y, Enemy::DIMENSION, RED);
        }
    }

    fn set_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
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

        if adjusted_y > -self.dimensions.y && adjusted_y < screen_height() + self.dimensions.y {
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

    fn update(&mut self, player: &mut Player, platforms: &mut [Plat], enemies: &mut [Enemy]) {

        self.scroll_pos += 0.1 * if self.scroll_pos <= 0.0 && player.adjusted_y() > screen_height() * 0.6 {0.0} else {screen_height() * 0.6 - player.adjusted_y()};

        player.set_scroll(self.scroll_pos);
        for plat in platforms {
            plat.set_scroll(self.scroll_pos);
        }
        for enemy in enemies {
            enemy.set_scroll(self.scroll_pos);
        }
    }
}

fn generate_platforms() -> Vec<Plat> {
    let mut platforms = vec![];

    // platforms.append(&mut vec![Plat::new(screen_width() - 100., screen_height() - 100., 50., 200.)]);
    // platforms.append(&mut vec![Plat::new(screen_width() - 150., screen_height() - 100., 50., 150.)]);
    // platforms.append(&mut vec![Plat::new(screen_width() -200., screen_height() - 100., 50., 100.)]);
    // platforms.append(&mut vec![Plat::new(screen_width() - 250., screen_height() - 100., 50., 50.)]);
    // platforms.append(&mut vec![Plat::new(screen_width() - 300., screen_height() - 100., 50., 0.)]);

    platforms.append(&mut vec![Plat::new(0., screen_height() - 100., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 200., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width(), screen_height() - 300., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 400., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(0., screen_height() - 500., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 600., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width(), screen_height() - 700., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 800., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(0., screen_height() - 900., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 1000., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width(), screen_height() - 1100., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 1200., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(0., screen_height() - 1300., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 1400., 150., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width(), screen_height() - 1500., 300., 25.)]);
    platforms.append(&mut vec![Plat::new(screen_width() - 400., screen_height() - 1600., 150., 25.)]);


    platforms
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut enemies = vec![Enemy::new(200.0, 200.0, 800.0)];
    let mut platforms = generate_platforms();
    let mut screen = Screen::new();

    // player.set_plats(platforms);
    // screen.set_plats(platforms);    

    loop {
        clear_background(DARKBLUE);
        
        player.update(&platforms);

        for enemy in &mut enemies {
            enemy.update(&platforms, &player);
        }

        for platform in &mut platforms {
            platform.update();
        }

        screen.update(&mut player, &mut platforms, &mut enemies);

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