mod utils;

use std::f64::consts::PI;

use rand::Rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Player {
    x: f64,
    y: f64,
    speed: f64,
    acceleration: f64,
    angle: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
}

// getters
#[wasm_bindgen]
impl Player {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 50.0,
            y: 50.0,
            speed: 0.0,
            acceleration: 0.2,
            angle: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: 25.0,
        }
    }
    pub fn accelerate_up(&mut self) {
        self.y_speed -= self.acceleration;
        self.calculate_speed();
    }
    pub fn accelerate_down(&mut self) {
        self.y_speed += self.acceleration;
        self.calculate_speed();
    }
    pub fn accelerate_left(&mut self) {
        self.x_speed -= self.acceleration;
        self.calculate_speed();
    }
    pub fn accelerate_right(&mut self) {
        self.x_speed += self.acceleration;
        self.calculate_speed();
    }
    pub fn calculate_speed(&mut self) {
        self.speed = js_sys::Math::sqrt(self.x_speed * self.speed + self.y_speed * self.y_speed);
        self.angle = (self.x_speed / self.speed).acos();
    }
    pub fn decelerate(&mut self) {
        if self.speed <= self.acceleration {
            self.speed = 0.0;
        } else {
            self.speed -= self.acceleration;
        }
        self.calculate_xyspeeds();
    }
    pub fn calculate_xyspeeds(&mut self) {
        self.x_speed = self.speed * (PI * (self.angle / 180.0)).cos();
        self.y_speed = self.speed * (PI * (self.angle / 180.0)).sin();
    }
    pub fn tick(&mut self, width: u32, height: u32) {
        let mut new_x = self.x + self.x_speed;
        let mut new_y = self.y + self.y_speed;

        if new_x - self.radius < 0.0 { // left wall collision
            new_x = 0.0 + self.radius;
            self.x_speed = 0.0;
            self.calculate_speed();
        } else if new_x + self.radius > width as f64 { // rigth wall collision
            new_x = width as f64 - self.radius;
            self.x_speed = 0.0;
            self.calculate_speed();
        } else if new_y - self.radius < 0.0 { // top wall collision
            new_y = 0.0 + self.radius;
            self.y_speed = 0.0;
            self.calculate_speed();
        } else if new_y + self.radius > height as f64 { // bottom wall collision
            new_y = height as f64 - self.radius;
            self.y_speed = 0.0;
            self.calculate_speed();
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    
}

#[wasm_bindgen]
pub struct Ball {
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
}

// getters
#[wasm_bindgen]
impl Ball {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[wasm_bindgen]
impl Ball {
    pub fn new() -> Ball {
        let mut ball = Ball {
            x: 200.0,
            y: 200.0,
            speed: 0.0,
            angle: 225.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: 10.0,
        };
        ball.calculate_xyspeeds();
        ball
    }
    pub fn calculate_xyspeeds(&mut self) {
        self.x_speed = self.speed * (PI * (self.angle / 180.0)).cos();
        self.y_speed = self.speed * (PI * (self.angle / 180.0)).sin();
    }
    pub fn tick(&mut self, width: u32, height: u32, wall_hit_speed_modifier: f64, resistances: f64) {
        let new_x = self.x + self.x_speed;
        let new_y = self.y + self.y_speed;

        let hit_angle ;
        if new_x - self.radius < 0.0 { // left wall collision
            if self.angle > 180.0 {
                hit_angle = 270.0 - self.angle;
                self.angle += 2.0 * hit_angle;
            } else {
                hit_angle = self.angle - 90.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();

        } else if new_x + self.radius > width as f64 { // rigth wall collision
            if self.angle < 90.0 {
                hit_angle = 90.0 - self.angle;
                self.angle = 90.0 + hit_angle;
            } else {
                hit_angle = self.angle - 270.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();

        } else if new_y - self.radius < 0.0 { // top wall collision
            if self.angle < 270.0 {
                hit_angle = self.angle - 180.0;
                self.angle = 180.0 - hit_angle;
            } else {
                hit_angle = 360.0 - self.angle;
                self.angle = hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();

        } else if new_y + self.radius > height as f64 { // bottom wall collision
            if self.angle < 90.0 {
                hit_angle = self.angle;
                self.angle = 360.0 - hit_angle;
            } else {
                hit_angle = 180.0 - self.angle;
                self.angle = 180.0 + hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();

        } else {
            self.x = new_x;
            self.y = new_y;
        }

        self.speed *= resistances;
        self.x_speed *= resistances;
        self.y_speed *= resistances;
    }

    pub fn randomize(&mut self) {
        self.speed = 6.0;
        self.angle = rand::thread_rng().gen_range(0.0, 360.0);
        self.calculate_xyspeeds();
    }
}

#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    pitch_line_width: u32,
    ball: Ball,
    wall_hit_speed_modifier: f64,
    resistances: f64,
    player: Player,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            width: 1000,
            height: 500,
            pitch_line_width: 5,
            ball: Ball::new(),
            wall_hit_speed_modifier: 0.8,
            resistances: 0.99,
            player: Player::new(),
        }
    }
    pub fn tick(&mut self) {
        self.ball.tick(self.width, self.height, self.wall_hit_speed_modifier, self.resistances);
        self.player.tick(self.width, self.height);

    }
}

// getters
#[wasm_bindgen]
impl Game {
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn pitch_line_width(&self) -> u32 { self.pitch_line_width }
    pub fn ball_x(&self) -> f64 { self.ball.x() }
    pub fn ball_y(&self) -> f64 { self.ball.y() }
    pub fn ball_radius(&self) -> f64 { self.ball.radius() }
    pub fn ball_randomize(&mut self) { self.ball.randomize(); }
    pub fn player_x(&self) -> f64 { self.player.x() }
    pub fn player_y(&self) -> f64 { self.player.y() }
    pub fn player_radius(&self) -> f64 { self.player.radius() }
    pub fn player_accelerate_up(&mut self) { self.player.accelerate_up(); }
    pub fn player_accelerate_down(&mut self) { self.player.accelerate_down(); }
    pub fn player_accelerate_left(&mut self) { self.player.accelerate_left(); }
    pub fn player_accelerate_right(&mut self) { self.player.accelerate_right(); }
    pub fn player_decelerate(&mut self) { self.player.decelerate(); }
}
