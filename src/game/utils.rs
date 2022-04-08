use crate::game::constants::{PITCH_LEFT_LINE, PITCH_RIGHT_LINE, PLAYER_DIAMETER, STADIUM_HEIGHT};
use crate::game::input::PlayerInput;
use rapier2d::dynamics::{RigidBodyHandle, RigidBodySet};
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

pub fn angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    const RADIAN: f32 = 180.0 / std::f32::consts::PI;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = f32::sqrt(dx * dx + dy * dy);
    RADIAN * (dx / dist).acos() * num::signum(dy)
}

#[derive(Clone)]
pub struct Player {
    pub rigid_body_handle: RigidBodyHandle,
    pub radius: f32,
    pub red: bool,
    pub number: usize,
    pub current_input: PlayerInput,
    pub last_tick_shot: bool,
}

impl Player {
    pub fn new(
        rigid_body_handle: RigidBodyHandle,
        radius: f32,
        red: bool,
        number: usize,
    ) -> Player {
        Player {
            rigid_body_handle,
            radius,
            red,
            number,
            current_input: PlayerInput::default(),
            last_tick_shot: false,
        }
    }

    pub fn set_last_tick_shot(&mut self, shot: bool) {
        self.last_tick_shot = shot;
    }

    pub fn to_circle(&self, rigid_body_set: &RigidBodySet) -> Circle {
        let rb = &rigid_body_set[self.rigid_body_handle];
        Circle::new(
            rb.translation().x,
            rb.translation().y,
            self.radius,
            self.red,
            self.number as i32,
        )
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        self.current_input = input;
    }

    pub fn get_input(&self) -> PlayerInput {
        self.current_input
    }

    pub fn reset_position(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        x_offset: f32,
        y_offset: f32,
    ) {
        let mut x = x_offset;
        let mut y = y_offset;
        if self.red {
            x += PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER;
            y += STADIUM_HEIGHT / 2.0
        } else {
            x += PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER;
            y += STADIUM_HEIGHT / 2.0
        }
        let player_body = &mut rigid_body_set[self.rigid_body_handle];
        player_body.set_position(Isometry::new(vector![x, y], 0.0), false);
        player_body.set_linvel(vector![0.0, 0.0], false);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub red: bool,
    pub player_number: i32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32, red: bool, player_number: i32) -> Circle {
        Circle {
            x,
            y,
            radius,
            red,
            player_number,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Edge {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) white: bool,
}

impl Edge {
    pub fn new(x_center: f32, y_center: f32, width: f32, height: f32, white: bool) -> Edge {
        Edge {
            x: x_center - width / 2.0,
            y: y_center - height / 2.0,
            width,
            height,
            white,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub(crate) red_score: u32,
    pub(crate) blue_score: u32,
}

impl Score {
    pub fn new(red_score: u32, blue_score: u32) -> Score {
        Score {
            red_score,
            blue_score,
        }
    }
}
pub struct Arbiter {
    pub red_scored: bool,
    pub blue_scored: bool,
    pub red_score: u32,
    pub blue_score: u32,
    pub send_score_message: bool,
    pub reset_timer: u32,
    pub game_ended: bool,
}

impl Arbiter {
    pub fn new() -> Arbiter {
        Arbiter {
            red_scored: false,
            blue_scored: false,
            red_score: 0,
            blue_score: 0,
            send_score_message: false,
            reset_timer: 0,
            game_ended: false,
        }
    }
    pub fn set_red_scored(&mut self) {
        self.red_scored = true;
        self.red_score += 1;
        self.send_score_message = true;
    }
    pub fn set_blue_scored(&mut self) {
        self.blue_scored = true;
        self.blue_score += 1;
        self.send_score_message = true;
    }
    pub fn reset_who_scored(&mut self) {
        self.blue_scored = false;
        self.red_scored = false;
    }
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    GameInit {
        edges: Vec<Edge>,
        goal_posts: Vec<Circle>,
        players: Vec<Circle>,
        ball: Circle,
    },
    GameState {
        players: Vec<Circle>,
        ball: Circle,
    },
    GoalScored {
        red_scored: bool,
        score: Score,
    },
    GameEnded,
}
