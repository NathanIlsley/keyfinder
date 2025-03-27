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
    platforms: Vec<Plat>,
    grounded: bool,
    jump_time: f32,
}

impl Player {
    const GRAVITY: f32 = 2000.0;
    const X_MOVEMENT_SPEED: f32 = 400.0;
    const X_RESPONSIVENESS: f32 = 2000.0;
    const JUMP_SPEED: f32 = 700.0;
    
    const CIRCLE_RADIUS: f32 = 16.0;

    fn new() -> Self {
        Self {
            position: Vector2 { x: screen_width() / 2.0, y: screen_height() / 2.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            platforms: vec![],
            grounded: false,
            jump_time: 0.0,
        }
    }

    fn set_plats(&mut self, plats: Vec<Plat>) {
        self.platforms = plats;
    }
    
    fn update(&mut self) {
        let delta_time = get_frame_time();

        // Inputs
        self.velocity.x += match (is_key_down(KeyCode::D), is_key_down(KeyCode::A)) {
            (true, false) => Player::X_RESPONSIVENESS * delta_time,
            (false, true) => -Player::X_RESPONSIVENESS * delta_time,
            _             => Player::X_RESPONSIVENESS * if self.velocity.x.abs() > 40.0 {-self.velocity.x / self.velocity.x.abs()} else {self.velocity.x = 0.0; 0.0} * delta_time,
        };

        self.velocity.x = clamp(self.velocity.x, -Player::X_MOVEMENT_SPEED, Player::X_MOVEMENT_SPEED);

        // self.jump_time += delta_time;

        if is_key_down(KeyCode::W) && self.jump_time <= 0.2 {
            self.velocity.y = -Player::JUMP_SPEED;
        }
        
        if self.grounded {
            self.jump_time = 0.0;
        } else {
            self.jump_time += delta_time;
        }

        // Apply gravity
        self.velocity.y += Player::GRAVITY * delta_time;

        // Update Position
        self.position = self.position.add(&self.velocity.mul(delta_time));

        // Check Collisions
        self.grounded = false;

        if self.position.clamp(Player::CIRCLE_RADIUS, screen_width() - Player::CIRCLE_RADIUS, Player::CIRCLE_RADIUS, screen_height() - Player::CIRCLE_RADIUS).y != 0.0 {
            self.velocity.y = 0.0;
            
            if self.position.y == screen_height() - Player::CIRCLE_RADIUS {
                self.grounded = true;
            }
        }

        let mut dist: Vector2;
        let mut overlap: Vector2;

        for plat in &self.platforms {
            dist = self.position.sub(&plat.position);
            if dist.x.abs() < plat.dimensions.x / 2.0 + Player::CIRCLE_RADIUS && dist.y.abs() < plat.dimensions.y / 2.0 + Player::CIRCLE_RADIUS {
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
        draw_circle(self.position.x, self.position.y, Player::CIRCLE_RADIUS, YELLOW);

    }
}

#[derive(Clone, Copy)]
struct Plat {
    position: Vector2,
    dimensions: Vector2,
}

impl Plat {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { position: Vector2::new(x, y) , dimensions: Vector2::new(w, h)}
    }

    fn update(&mut self) {
        // Draw on screen
        draw_rectangle(self.position.x - self.dimensions.x / 2.0, self.position.y - self.dimensions.y / 2.0, self.dimensions.x, self.dimensions.y, WHITE);
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut platform1 = Plat::new(200.0, 300.0, 300.0, 25.0);
    let mut platform2 = Plat::new(500.0, 100.0, 300.0, 25.0);
    let mut platform3 = Plat::new(650.0, 400.0, 300.0, 25.0);
    let platforms = vec![platform1, platform2, platform3];

    player.set_plats(platforms);

    loop {
        clear_background(DARKBLUE);

        player.update();
        platform1.update();
        platform2.update();
        platform3.update();

        // println!("{}", get_fps());

        next_frame().await;
    }
}