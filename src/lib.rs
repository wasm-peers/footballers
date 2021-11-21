mod utils;

use wasm_bindgen::prelude::*;
use std::f64::consts::PI;
use rand::Rng;
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const RADIAN: f64 = 57.2958;

pub struct Player {
    x: f64,
    y: f64,
    speed: f64,
    top_speed: f64,
    acceleration: f64,
    deceleration: f64,
    angle: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
}

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
    pub fn speed(&self) -> f64 {
        self.speed
    }
    pub fn x_speed(&self) -> f64 {
        self.x_speed
    }
    pub fn y_speed(&self) -> f64 {
        self.y_speed
    }
    pub fn angle(&self) -> f64 {
        self.angle
    }
    pub fn new() -> Player {
        Player {
            x: 50.0,
            y: 50.0,
            speed: 0.0,
            top_speed: 3.0,
            acceleration: 0.15,
            deceleration: 0.1,
            angle: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: 25.0,
        }
    }
    pub fn accelerate_up(&mut self) {
        self.y_speed -= self.acceleration;
        self.calculate_speed();

        if self.speed > self.top_speed {
            self.y_speed = -self.top_speed * (PI * (self.angle / 180.0)).sin().abs() ;
        }
        // if self.y_speed < -self.top_speed {
        //     self.y_speed = -self.top_speed;
        // }
        self.calculate_speed();
    }
    pub fn accelerate_down(&mut self) {
        self.y_speed += self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.y_speed = self.top_speed * (PI * (self.angle / 180.0)).sin().abs();
        }
        // if self.y_speed > self.top_speed {
        //     self.y_speed = self.top_speed;
        // }
        self.calculate_speed();
    }
    pub fn accelerate_left(&mut self) {
        self.x_speed -= self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.x_speed = -self.top_speed * (PI * (self.angle / 180.0)).cos().abs();
        }
        // if self.x_speed < -self.top_speed {
        //     self.x_speed = -self.top_speed;
        // }
        self.calculate_speed();
    }
    pub fn accelerate_right(&mut self) {
        self.x_speed += self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.x_speed = self.top_speed * (PI * (self.angle / 180.0)).cos().abs();
        }
        // if self.x_speed > self.top_speed {
        //     self.x_speed = self.top_speed;
        // }
        self.calculate_speed();
    }
    pub fn calculate_speed(&mut self) {
        self.speed = js_sys::Math::sqrt(self.x_speed * self.x_speed + self.y_speed * self.y_speed);
        if self.speed != 0.0 {
            self.angle = RADIAN * (self.x_speed / self.speed).acos() * num::signum(self.y_speed);
        }
    }
    // pub fn decelerate(&mut self) {
    //     if self.speed <= self.acceleration {
    //         self.speed = 0.0;
    //     } else {
    //         self.speed -= self.acceleration;
    //     }
    //     self.calculate_xyspeeds();
    // }
    pub fn x_decelerate(&mut self) {
        if self.x_speed.signum() == -1.0 {
            if -self.x_speed <= self.deceleration {
            // if -self.x_speed <= self.deceleration * (PI * (self.angle / 180.0)).cos() {
                self.x_speed = 0.0;
            } else {
                self.x_speed += self.deceleration;
            }
        } else {
            if self.x_speed <= self.deceleration {
            // if self.x_speed <= self.deceleration * (PI * (self.angle / 180.0)).cos() {
                self.x_speed = 0.0;
            } else {
                self.x_speed -= self.deceleration;
            }
        }
        // self.calculate_xyspeeds();
        self.calculate_speed();
    }
    pub fn y_decelerate(&mut self) {
        if self.y_speed.signum() == -1.0 {
            if -self.y_speed <= self.deceleration {
            // if -self.y_speed <= self.deceleration * (PI * (self.angle / 180.0)).sin() {
                self.y_speed = 0.0;
            } else {
                self.y_speed += self.deceleration;
            }
        } else {
            if self.y_speed <= self.deceleration {
            // if self.y_speed <= self.deceleration * (PI * (self.angle / 180.0)).sin() {
                self.y_speed = 0.0;
            } else {
                self.y_speed -= self.deceleration;
            }
        }
        // self.calculate_xyspeeds();
        self.calculate_speed();
    }
    // pub fn calculate_xyspeeds(&mut self) {
    //     self.x_speed = self.speed * (PI * (self.angle / 180.0)).cos();
    //     self.y_speed = self.speed * (PI * (self.angle / 180.0)).sin();
    // }
    pub fn tick(&mut self, width: u32, height: u32) {
        let mut new_x = self.x + self.x_speed;
        let mut new_y = self.y + self.y_speed;

        let mut left_right_collision = false;
        let mut up_down_collision = false;

        if new_x - self.radius < 0.0 {
            // left wall collision
            self.x = 0.0 + self.radius;
            self.x_speed = 0.0;
            self.y = new_y;
            self.calculate_speed();
            left_right_collision = true;
        } else if new_x + self.radius > width as f64 {
            // rigth wall collision
            self.x = width as f64 - self.radius;
            self.x_speed = 0.0;
            self.y = new_y;
            self.calculate_speed();
            left_right_collision = true;
        }
        if new_y - self.radius < 0.0 {
            // top wall collision
            self.y = 0.0 + self.radius;
            self.y_speed = 0.0;
            self.x = new_x;
            self.calculate_speed();
            up_down_collision = true;
        } else if new_y + self.radius > height as f64 {
            // bottom wall collision
            self.y = height as f64 - self.radius;
            self.y_speed = 0.0;
            self.x = new_x;
            self.calculate_speed();
            up_down_collision = true;
        }
        if !left_right_collision && !up_down_collision {
            self.x = new_x;
            self.y = new_y;
        }
    }
}

pub struct Ball {
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
}

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
    pub fn tick(
        &mut self,
        width: u32,
        height: u32,
        wall_hit_speed_modifier: f64,
        resistances: f64,
    ) {
        let new_x = self.x + self.x_speed;
        let new_y = self.y + self.y_speed;

        let hit_angle;
        if new_x - self.radius < 0.0 {
            // left wall collision
            if self.angle > 180.0 {
                hit_angle = 270.0 - self.angle;
                self.angle += 2.0 * hit_angle;
            } else {
                hit_angle = self.angle - 90.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();
        } else if new_x + self.radius > width as f64 {
            // rigth wall collision
            if self.angle < 90.0 {
                hit_angle = 90.0 - self.angle;
                self.angle = 90.0 + hit_angle;
            } else {
                hit_angle = self.angle - 270.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();
        } else if new_y - self.radius < 0.0 {
            // top wall collision
            if self.angle < 270.0 {
                hit_angle = self.angle - 180.0;
                self.angle = 180.0 - hit_angle;
            } else {
                hit_angle = 360.0 - self.angle;
                self.angle = hit_angle;
            }

            self.speed *= wall_hit_speed_modifier;
            self.calculate_xyspeeds();
        } else if new_y + self.radius > height as f64 {
            // bottom wall collision
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
    last_tick_shot: bool,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            width: 700,
            height: 500,
            pitch_line_width: 5,
            ball: Ball::new(),
            wall_hit_speed_modifier: 0.8,
            resistances: 0.99,
            player: Player::new(),
            last_tick_shot: false,
        }
    }
    pub fn tick(&mut self, val: &JsValue) {
        let input: PlayerInput = val.into_serde().unwrap();
        self.move_player(&input);
        self.ball.tick(
            self.width,
            self.height,
            self.wall_hit_speed_modifier,
            self.resistances,
        );
        self.player.tick(self.width, self.height);
    }
    fn move_player(&mut self, input: &PlayerInput) {
        if input.shoot {
            if !self.last_tick_shot {
                self.ball_randomize();
                self.last_tick_shot = true;
            }
        } else {
            self.last_tick_shot = false;
        }
        if input.up {
            self.player_accelerate_up();
        } else if input.down {
            self.player_accelerate_down();
        } else {
            self.player.y_decelerate();
        }
        if input.left {
            self.player_accelerate_left();
        } else if input.right {
            self.player_accelerate_right();
        } else {
            self.player.x_decelerate();
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pitch_line_width(&self) -> u32 {
        self.pitch_line_width
    }
    pub fn ball_x(&self) -> f64 {
        self.ball.x()
    }
    pub fn ball_y(&self) -> f64 {
        self.ball.y()
    }
    pub fn ball_radius(&self) -> f64 {
        self.ball.radius()
    }
    pub fn ball_randomize(&mut self) {
        self.ball.randomize();
    }
    pub fn player_x(&self) -> f64 {
        self.player.x()
    }
    pub fn player_y(&self) -> f64 {
        self.player.y()
    }
    pub fn player_radius(&self) -> f64 {
        self.player.radius()
    }
    pub fn player_speed(&self) -> f64 {
        self.player.speed()
    }
    pub fn player_x_speed(&self) -> f64 {
        self.player.x_speed()
    }
    pub fn player_y_speed(&self) -> f64 {
        self.player.y_speed()
    }
    pub fn player_angle(&self) -> f64 {
        self.player.angle()
    }
    pub fn player_accelerate_up(&mut self) {
        self.player.accelerate_up();
    }
    pub fn player_accelerate_down(&mut self) {
        self.player.accelerate_down();
    }
    pub fn player_accelerate_left(&mut self) {
        self.player.accelerate_left();
    }
    pub fn player_accelerate_right(&mut self) {
        self.player.accelerate_right();
    }
    // pub fn player_decelerate(&mut self) {
    //     self.player.decelerate();
    // }
}

#[derive(Serialize, Deserialize)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shoot: bool
}