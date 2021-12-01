mod utils;

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn alert(s: &str);
}

const RADIAN: f64 = 57.2958;
const PLAYER_DIAMETER: u32 = 40;
const GOAL_LENGTH: u32 = 150;
const RESISTANCES: f64 = 0.99;
const WALL_HIT_SPEED_MODIFIER: f64 = 0.8;

const PITCH_LEFT_WALL: u32 = 0 + PLAYER_DIAMETER;
const PITCH_RIGHT_WALL: u32 = PITCH_LEFT_WALL + 700;
const PITCH_TOP_WALL: u32 = 0 + PLAYER_DIAMETER;
const PITCH_BOTTOM_WALL: u32 = PITCH_TOP_WALL + 500;
const STADIUM_LEFT_WALL: u32 = 0;
const STADIUM_RIGHT_WALL: u32 = PITCH_RIGHT_WALL + PLAYER_DIAMETER;
const STADIUM_TOP_WALL: u32 = 0;
const STADIUM_BOTTOM_WALL: u32 = PITCH_BOTTOM_WALL + PLAYER_DIAMETER;


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
            x: (STADIUM_RIGHT_WALL - STADIUM_LEFT_WALL) as f64 * 0.4,
            y: ((STADIUM_BOTTOM_WALL - STADIUM_TOP_WALL) / 2) as f64,
            speed: 0.0,
            top_speed: 3.0,
            acceleration: 0.15,
            deceleration: 0.1,
            angle: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: (PLAYER_DIAMETER / 2) as f64,
        }
    }
    pub fn accelerate_up(&mut self) {
        self.y_speed -= self.acceleration;
        self.calculate_speed();

        if self.speed > self.top_speed {
            self.y_speed = -self.top_speed * (PI * (self.angle / 180.0)).sin().abs();
        }
        self.calculate_speed();
    }
    pub fn accelerate_down(&mut self) {
        self.y_speed += self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.y_speed = self.top_speed * (PI * (self.angle / 180.0)).sin().abs();
        }
        self.calculate_speed();
    }
    pub fn accelerate_left(&mut self) {
        self.x_speed -= self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.x_speed = -self.top_speed * (PI * (self.angle / 180.0)).cos().abs();
        }
        self.calculate_speed();
    }
    pub fn accelerate_right(&mut self) {
        self.x_speed += self.acceleration;
        self.calculate_speed();
        if self.speed > self.top_speed {
            self.x_speed = self.top_speed * (PI * (self.angle / 180.0)).cos().abs();
        }
        self.calculate_speed();
    }
    pub fn calculate_speed(&mut self) {
        self.speed = js_sys::Math::sqrt(self.x_speed * self.x_speed + self.y_speed * self.y_speed);
        if self.speed != 0.0 {
            self.angle = RADIAN * (self.x_speed / self.speed).acos() * num::signum(self.y_speed);
        }
    }
    pub fn x_decelerate(&mut self) {
        if self.x_speed.signum() == -1.0 {
            if -self.x_speed <= self.deceleration {
                self.x_speed = 0.0;
            } else {
                self.x_speed += self.deceleration;
            }
        } else {
            if self.x_speed <= self.deceleration {
                self.x_speed = 0.0;
            } else {
                self.x_speed -= self.deceleration;
            }
        }
        self.calculate_speed();
    }
    pub fn y_decelerate(&mut self) {
        if self.y_speed.signum() == -1.0 {
            if -self.y_speed <= self.deceleration {
                self.y_speed = 0.0;
            } else {
                self.y_speed += self.deceleration;
            }
        } else {
            if self.y_speed <= self.deceleration {
                self.y_speed = 0.0;
            } else {
                self.y_speed -= self.deceleration;
            }
        }
        self.calculate_speed();
    }
    pub fn tick(&mut self) {
        let new_x = self.x + self.x_speed;
        let new_y = self.y + self.y_speed;

        if new_x - self.radius < STADIUM_LEFT_WALL as f64 {
            // left wall collision
            if new_y - self.radius < STADIUM_TOP_WALL as f64 {
                // top wall collision
                self.x = STADIUM_LEFT_WALL as f64 + self.radius;
                self.y = STADIUM_LEFT_WALL as f64 + self.radius;
                self.x_speed = 0.0;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else if new_y + self.radius > STADIUM_BOTTOM_WALL as f64 {
                // bottom wall collision
                self.x = STADIUM_LEFT_WALL as f64 + self.radius;
                self.y = STADIUM_BOTTOM_WALL as f64 - self.radius;
                self.x_speed = 0.0;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else {
                // without top or bottom wall collision
                self.x = STADIUM_LEFT_WALL as f64 + self.radius;
                self.y = new_y;
                self.x_speed = 0.0;
                self.speed = 0.0;
            }
        } else if new_x + self.radius > STADIUM_RIGHT_WALL as f64 {
            // rigth wall collision
            if new_y - self.radius < STADIUM_TOP_WALL as f64 {
                // top wall collision
                self.x = STADIUM_RIGHT_WALL as f64 - self.radius;
                self.y = STADIUM_TOP_WALL as f64 + self.radius;
                self.x_speed = 0.0;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else if new_y + self.radius > STADIUM_BOTTOM_WALL as f64 {
                // bottom wall collision
                self.x = STADIUM_RIGHT_WALL as f64 - self.radius;
                self.y = STADIUM_BOTTOM_WALL as f64 - self.radius;
                self.x_speed = 0.0;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else {
                // without top or bottom wall collision
                self.x = STADIUM_RIGHT_WALL as f64 - self.radius;
                self.y = new_y;
                self.x_speed = 0.0;
                self.speed = 0.0;
            }
        } else {
            // without left or right wall collision
            if new_y - self.radius < STADIUM_TOP_WALL as f64 {
                // top wall collision
                self.x = new_x;
                self.y = STADIUM_TOP_WALL as f64 + self.radius;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else if new_y + self.radius > STADIUM_BOTTOM_WALL as f64 {
                // bottom wall collision
                self.x = new_x;
                self.y = STADIUM_BOTTOM_WALL as f64 - self.radius;
                self.y_speed = 0.0;
                self.speed = 0.0;
            } else {
                // without any collision on the map
                self.x = new_x;
                self.y = new_y;
            }
        }
    }
}

pub struct Ball {
    x: f64,
    y: f64,
    speed: f64,
    top_speed: f64,
    angle: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
    shoot_range: f64,
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
            x: ((STADIUM_RIGHT_WALL - STADIUM_LEFT_WALL) / 2) as f64,
            y: ((STADIUM_BOTTOM_WALL - STADIUM_TOP_WALL) / 2) as f64,
            speed: 0.0,
            top_speed: 6.0,
            angle: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: 10.0,
            shoot_range: 5.0,
        };
        ball.calculate_xyspeeds();
        ball
    }
    pub fn calculate_xyspeeds(&mut self) {
        self.x_speed = self.speed * (PI * (self.angle / 180.0)).cos();
        self.y_speed = self.speed * (PI * (self.angle / 180.0)).sin();
    }
    pub fn tick(&mut self) {
        let new_x = self.x + self.x_speed;
        let new_y = self.y + self.y_speed;

        let hit_angle;
        if new_x - self.radius < PITCH_LEFT_WALL as f64 {
            // left wall collision
            if self.angle > 180.0 {
                hit_angle = 270.0 - self.angle;
                self.angle += 2.0 * hit_angle;
            } else {
                hit_angle = self.angle - 90.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= WALL_HIT_SPEED_MODIFIER;
            self.calculate_xyspeeds();
        } else if new_x + self.radius > PITCH_RIGHT_WALL as f64 && !(new_y + self.radius > ((STADIUM_BOTTOM_WALL - GOAL_LENGTH) / 2) as f64 && new_y + self.radius < ((STADIUM_BOTTOM_WALL + GOAL_LENGTH) / 2) as f64) {
            // rigth wall collision
            if self.angle < 90.0 {
                hit_angle = 90.0 - self.angle;
                self.angle = 90.0 + hit_angle;
            } else {
                hit_angle = self.angle - 270.0;
                self.angle -= 2.0 * hit_angle;
            }

            self.speed *= WALL_HIT_SPEED_MODIFIER;
            self.calculate_xyspeeds();
        } else if new_y - self.radius < PITCH_TOP_WALL as f64 {
            // top wall collision
            if self.angle < 270.0 {
                hit_angle = self.angle - 180.0;
                self.angle = 180.0 - hit_angle;
            } else {
                hit_angle = 360.0 - self.angle;
                self.angle = hit_angle;
            }

            self.speed *= WALL_HIT_SPEED_MODIFIER;
            self.calculate_xyspeeds();
        } else if new_y + self.radius > PITCH_BOTTOM_WALL as f64 {
            // bottom wall collision
            if self.angle < 90.0 {
                hit_angle = self.angle;
                self.angle = 360.0 - hit_angle;
            } else {
                hit_angle = 180.0 - self.angle;
                self.angle = 180.0 + hit_angle;
            }

            self.speed *= WALL_HIT_SPEED_MODIFIER;
            self.calculate_xyspeeds();
        } else {
            self.x = new_x;
            self.y = new_y;
        }

        self.speed *= RESISTANCES;
        self.x_speed *= RESISTANCES;
        self.y_speed *= RESISTANCES;
    }
    pub fn shoot(&mut self, player: &Player) {
        if distance(player.x(), player.y(), self.x, self.y) > player.radius() + self.radius + self.shoot_range {
            return;
        }
        self.angle = angle(player.x(), player.y(), self.x, self.y);
        self.speed = self.top_speed;
        self.calculate_xyspeeds();
    }
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    js_sys::Math::sqrt(dx * dx + dy * dy)
}

fn angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = js_sys::Math::sqrt(dx * dx + dy * dy);
    RADIAN * (dx / dist).acos() * num::signum(dy)
}

pub struct GoalBlue {
}

impl GoalBlue {
    pub fn new() -> GoalBlue {
        GoalBlue {
        }
    }
    pub fn passed_through(&self, ball: &Ball) -> bool {
        ball.x() > PITCH_RIGHT_WALL as f64 + ball.radius()
    }
}

#[wasm_bindgen]
pub struct Game {
    ball: Ball,
    player: Player,
    last_tick_shot: bool,
    goal_blue: GoalBlue,
    game_interrupted: bool,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            ball: Ball::new(),
            player: Player::new(),
            last_tick_shot: false,
            goal_blue: GoalBlue::new(),
            game_interrupted: false,
        }
    }
    pub fn tick(&mut self, val: &JsValue) {
        let input: PlayerInput = val.into_serde().unwrap();
        self.parse_player_input(&input);
        self.player.tick();
        self.ball.tick();
        if !self.game_interrupted && self.goal_blue.passed_through(&self.ball) {
            alert("Red Scored!");
            self.game_interrupted = true;
        }
    }
    fn parse_player_input(&mut self, input: &PlayerInput) {
        if input.shoot {
            if !self.last_tick_shot {
                self.ball.shoot(&self.player);
                self.last_tick_shot = true;
            }
        } else {
            self.last_tick_shot = false;
        }
        if input.up {
            self.player.accelerate_up();
        } else if input.down {
            self.player.accelerate_down();
        } else {
            self.player.y_decelerate();
        }
        if input.left {
            self.player.accelerate_left();
        } else if input.right {
            self.player.accelerate_right();
        } else {
            self.player.x_decelerate();
        }
    }
    pub fn pitch_left_wall(&self) -> u32 {
        PITCH_LEFT_WALL
    }
    pub fn pitch_right_wall(&self) -> u32 {
        PITCH_RIGHT_WALL
    }
    pub fn pitch_top_wall(&self) -> u32 {
        PITCH_TOP_WALL
    }
    pub fn pitch_bottom_wall(&self) -> u32 {
        PITCH_BOTTOM_WALL
    }
    pub fn stadium_left_wall(&self) -> u32 {
        STADIUM_LEFT_WALL
    }
    pub fn stadium_right_wall(&self) -> u32 {
        STADIUM_RIGHT_WALL
    }
    pub fn stadium_top_wall(&self) -> u32 {
        STADIUM_TOP_WALL
    }
    pub fn stadium_bottom_wall(&self) -> u32 {
        STADIUM_BOTTOM_WALL
    }
    pub fn goal_length(&self) -> u32 {
        GOAL_LENGTH
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
}

#[derive(Serialize, Deserialize)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shoot: bool,
}
