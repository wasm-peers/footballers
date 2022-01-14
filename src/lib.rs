mod utils;

use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rusty_games_library::{ NetworkManager, ConnectionType, SessionId};

#[wasm_bindgen(module = "/js/rendering.js")]
extern "C" {
    #[wasm_bindgen(js_name = initGame)]
    fn init_game_from_js();
    #[wasm_bindgen(js_name = draw)]
    fn draw_from_js();
    #[wasm_bindgen(js_name = tick)]
    fn tick_from_js();
}

#[wasm_bindgen(module = "/js/inputs.js")]
extern "C" {
    #[wasm_bindgen(js_name = getInputRed)]
    fn get_input_red_from_js() -> JsValue;
    #[wasm_bindgen(js_name = getInputBlue)]
    fn get_input_blue_from_js() -> JsValue;
}

// ==== constants ====

const PLAYER_DIAMETER: f32 = 30.0;
const PLAYER_RADIUS: f32 = PLAYER_DIAMETER / 2.0;
const PLAYER_ACCELERATION: f32 = 3_000.0;
const BALL_RADIUS: f32 = 10.0;
const PLAYER_TOP_SPEED: f32 = 110.0;
const BALL_TOP_SPEED: f32 = 200.0;
const SHOOTING_DISTANCE: f32 = PLAYER_RADIUS + BALL_RADIUS + BALL_RADIUS / 2.0;

const GOAL_BREADTH: f32 = 120.0;
const GOAL_DEPTH: f32 = 3.0 * BALL_RADIUS;
const PITCH_VERTICAL_LINE_HEIGHT: f32 = (PITCH_HEIGHT - GOAL_BREADTH) / 2.0;

const PITCH_WIDTH: f32 = 500.0;
const PITCH_HEIGHT: f32 = 300.0;
const PITCH_LINE_BREADTH: f32 = 3.0;
const PITCH_LEFT_LINE: f32 = 0.0 + 2.0 * PLAYER_DIAMETER;
const PITCH_RIGHT_LINE: f32 = PITCH_LEFT_LINE + PITCH_WIDTH;
const PITCH_TOP_LINE: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_BOTTOM_LINE: f32 = PITCH_TOP_LINE + PITCH_HEIGHT;
const STADIUM_WIDTH: f32 = 2.0 * PLAYER_DIAMETER + PITCH_WIDTH + 2.0 * PLAYER_DIAMETER;
const STADIUM_HEIGHT: f32 = 2.0 * PLAYER_DIAMETER + PITCH_HEIGHT;

const RESET_TIME: u32 = 60 * 2;

// ==== collision groups ====

const PITCH_LINES_GROUP: u32 = 0b_0000_0001;
const GOAL_POSTS_GROUP: u32 = 0b_0000_0010;
const PLAYERS_GROUP: u32 = 0b_0000_0100;
const STADIUM_WALLS_GROUP: u32 = 0b_0000_1000;
const BALL_GROUP: u32 = 0b_0001_0000;

// ==== helper functions ====

fn angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    const RADIAN: f32 = 180.0 / std::f32::consts::PI;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = f32::sqrt(dx * dx + dy * dy);
    RADIAN * (dx / dist).acos() * num::signum(dy)
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// ==== starting point called in index.js ====

#[wasm_bindgen]
pub fn main() {
    utils::set_panic_hook();
    init_game_from_js();
}

// ==== game class ====

#[wasm_bindgen]
pub struct Game {
    network_manager: NetworkManager,
    players: Vec<Player>,
    edges: Vec<Edge>,
    goals_posts: Vec<Circle>,
    red_player_body_handle: RigidBodyHandle,
    blue_player_body_handle: RigidBodyHandle,
    ball_body_handle: RigidBodyHandle,
    red_last_tick_shot: bool,
    blue_last_tick_shot: bool,
    red_scored: bool,
    blue_scored: bool,
    reset_timer: u32,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

#[wasm_bindgen]
impl Game {
    pub fn new(session_id: SessionId, is_host: bool) -> Game {
        let mut network_manager = NetworkManager::new(env!("WS_IP_PORT"), session_id, ConnectionType::Stun, is_host).expect("failed to create network manager");
        let mut players: Vec<Player> = Vec::new();
        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();
        let mut edges = Vec::new();
        let mut collider_set: ColliderSet = ColliderSet::new();
        let mut goals_posts = Vec::new();

        Game::create_pitch_lines(&mut collider_set, &mut edges);
        Game::create_goals_posts(&mut collider_set, &mut goals_posts);
        Game::create_stadium_walls(&mut collider_set);
        let player_handles =
            Game::create_players(&mut rigid_body_set, &mut collider_set, &mut players);
        let ball_body_handle = Game::create_ball(&mut rigid_body_set, &mut collider_set);
        Game {
            network_manager,
            players,
            edges,
            goals_posts,
            red_player_body_handle: player_handles.0,
            blue_player_body_handle: player_handles.1,
            ball_body_handle,
            red_last_tick_shot: false,
            blue_last_tick_shot: false,
            red_scored: false,
            blue_scored: false,
            reset_timer: 0,
            rigid_body_set,
            collider_set,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }
    fn create_pitch_lines(collider_set: &mut ColliderSet, edges: &mut Vec<Edge>) {
        let mut create_line_closure = |width, height, x, y, white, membership, filter| {
            let cuboid_collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0)
                .collision_groups(InteractionGroups::new(membership, filter))
                .translation(vector![x, y])
                .build();
            edges.push(Edge::new(
                cuboid_collider.translation().x,
                cuboid_collider.translation().y,
                width,
                height,
                white,
            ));
            collider_set.insert(cuboid_collider);
        };

        // left higher pitch line
        create_line_closure(
            PITCH_LINE_BREADTH,
            PITCH_VERTICAL_LINE_HEIGHT,
            PITCH_LEFT_LINE,
            (STADIUM_HEIGHT - GOAL_BREADTH - PITCH_VERTICAL_LINE_HEIGHT) / 2.0,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        // left lower pitch line
        create_line_closure(
            PITCH_LINE_BREADTH,
            PITCH_VERTICAL_LINE_HEIGHT,
            PITCH_LEFT_LINE,
            (STADIUM_HEIGHT + GOAL_BREADTH + PITCH_VERTICAL_LINE_HEIGHT) / 2.0,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        // left goal
        create_line_closure(
            PITCH_LINE_BREADTH,
            GOAL_BREADTH,
            PITCH_LEFT_LINE - GOAL_DEPTH,
            STADIUM_HEIGHT / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        create_line_closure(
            GOAL_DEPTH,
            PITCH_LINE_BREADTH,
            PITCH_LEFT_LINE - GOAL_DEPTH / 2.0,
            (STADIUM_HEIGHT - GOAL_BREADTH) / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        create_line_closure(
            GOAL_DEPTH,
            PITCH_LINE_BREADTH,
            PITCH_LEFT_LINE - GOAL_DEPTH / 2.0,
            (STADIUM_HEIGHT + GOAL_BREADTH) / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );

        // right higher pitch line
        create_line_closure(
            PITCH_LINE_BREADTH,
            PITCH_VERTICAL_LINE_HEIGHT,
            PITCH_RIGHT_LINE,
            (STADIUM_HEIGHT - GOAL_BREADTH - PITCH_VERTICAL_LINE_HEIGHT) / 2.0,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        // right lower pitch line
        create_line_closure(
            PITCH_LINE_BREADTH,
            PITCH_VERTICAL_LINE_HEIGHT,
            PITCH_RIGHT_LINE,
            (STADIUM_HEIGHT + GOAL_BREADTH + PITCH_VERTICAL_LINE_HEIGHT) / 2.0,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        // right goal
        create_line_closure(
            PITCH_LINE_BREADTH,
            GOAL_BREADTH,
            PITCH_RIGHT_LINE + GOAL_DEPTH,
            STADIUM_HEIGHT / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        create_line_closure(
            GOAL_DEPTH,
            PITCH_LINE_BREADTH,
            PITCH_RIGHT_LINE + GOAL_DEPTH / 2.0,
            (STADIUM_HEIGHT - GOAL_BREADTH) / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
        create_line_closure(
            GOAL_DEPTH,
            PITCH_LINE_BREADTH,
            PITCH_RIGHT_LINE + GOAL_DEPTH / 2.0,
            (STADIUM_HEIGHT + GOAL_BREADTH) / 2.0,
            false,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );

        // top pitch line`
        create_line_closure(
            PITCH_WIDTH,
            PITCH_LINE_BREADTH,
            STADIUM_WIDTH / 2.0,
            PITCH_TOP_LINE,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );

        // bottom pitch line
        create_line_closure(
            PITCH_WIDTH,
            PITCH_LINE_BREADTH,
            STADIUM_WIDTH / 2.0,
            PITCH_BOTTOM_LINE,
            true,
            PITCH_LINES_GROUP,
            PITCH_LINES_GROUP,
        );
    }
    fn create_goals_posts(collider_set: &mut ColliderSet, goals_posts: &mut Vec<Circle>) {
        let mut create_post_closure = |x, y, red| {
            let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
                .collision_groups(InteractionGroups::new(GOAL_POSTS_GROUP, GOAL_POSTS_GROUP))
                .translation(vector![x, y])
                .build();
            goals_posts.push(Circle::new(
                ball_collider.translation().x,
                ball_collider.translation().y,
                BALL_RADIUS,
                red,
                -1,
            ));
            collider_set.insert(ball_collider);
        };
        // left red goal
        create_post_closure(
            PITCH_LEFT_LINE,
            PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 - GOAL_BREADTH / 2.0,
            true,
        );
        create_post_closure(
            PITCH_LEFT_LINE,
            PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 + GOAL_BREADTH / 2.0,
            true,
        );

        // right blue goal
        create_post_closure(
            PITCH_RIGHT_LINE,
            PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 - GOAL_BREADTH / 2.0,
            false,
        );
        create_post_closure(
            PITCH_RIGHT_LINE,
            PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 + GOAL_BREADTH / 2.0,
            false,
        );
    }
    fn create_stadium_walls(collider_set: &mut ColliderSet) {
        let mut create_wall_closure = |width, height, x, y| {
            let cuboid_collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0)
                .collision_groups(InteractionGroups::new(
                    STADIUM_WALLS_GROUP,
                    STADIUM_WALLS_GROUP,
                ))
                .translation(vector![x, y])
                .build();
            collider_set.insert(cuboid_collider);
        };
        // left stadium wall
        create_wall_closure(0.0, STADIUM_HEIGHT, 0.0, STADIUM_HEIGHT / 2.0);

        // right stadium wall
        create_wall_closure(0.0, STADIUM_HEIGHT, STADIUM_WIDTH, STADIUM_HEIGHT / 2.0);

        // top stadium wall
        create_wall_closure(STADIUM_WIDTH, 0.0, STADIUM_WIDTH / 2.0, 0.0);

        // bottom stadium wall
        create_wall_closure(STADIUM_WIDTH, 0.0, STADIUM_WIDTH / 2.0, STADIUM_HEIGHT);
    }
    fn create_players(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        players: &mut Vec<Player>,
    ) -> (RigidBodyHandle, RigidBodyHandle) {
        const COLLISION_GROUP: u32 =
            PLAYERS_GROUP | STADIUM_WALLS_GROUP | BALL_GROUP | GOAL_POSTS_GROUP;
        let mut create_player_closure = |x, y, red, number| -> RigidBodyHandle {
            let player_rigid_body = RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .translation(vector![x, y])
                .build();
            let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
                .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
                .restitution(0.7)
                .build();
            let player_body_handle: RigidBodyHandle = rigid_body_set.insert(player_rigid_body);
            collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
            players.push(Player::new(player_body_handle, PLAYER_RADIUS, red, number));
            player_body_handle
        };
        // for i in 2..=2 {
        //     create_player_closure(
        //         PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER as f32,
        //         STADIUM_HEIGHT / 2.0 - PLAYER_DIAMETER + 2.0 * PLAYER_DIAMETER * i as f32,
        //         false,
        //         i,
        //     );
        // }
        (
            create_player_closure(
                PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER,
                STADIUM_HEIGHT / 2.0,
                true,
                1,
            ),
            create_player_closure(
                PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER,
                STADIUM_HEIGHT / 2.0,
                false,
                1,
            ),
        )
    }
    fn create_ball(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> RigidBodyHandle {
        const COLLISION_GROUP: u32 =
            BALL_GROUP | PLAYERS_GROUP | PITCH_LINES_GROUP | GOAL_POSTS_GROUP;

        let ball_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.5)
            .translation(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0])
            .build();
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
            .restitution(0.7)
            .build();
        let ball_body_handle: RigidBodyHandle = rigid_body_set.insert(ball_rigid_body);
        collider_set.insert_with_parent(ball_collider, ball_body_handle, rigid_body_set);

        return ball_body_handle;
    }
    pub fn start(&mut self) -> Result<(), JsValue> {

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            request_animation_frame(f.borrow().as_ref().unwrap());

            tick_from_js();
            draw_from_js();

            // if is_host { tick() send_state()}
            // else {}
        }) as Box<dyn FnMut()>));

        // let on_open_callback = move || {
        //     request_animation_frame(g.borrow().as_ref().unwrap());

        // };

        // let on_message_callback = |message: String| {

        // };
        // self.network_manager.start(on_open_callback, on_message_callback).expect("network manager failed to start");

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
    pub fn tick(&mut self) {
        if self.reset_timer > 0 {
            self.timer_tick();
        }

        let val_red = get_input_red_from_js();
        let val_blue = get_input_blue_from_js();
        let input_red: PlayerInput = val_red.into_serde().unwrap();
        let input_blue: PlayerInput = val_blue.into_serde().unwrap();
        self.parse_player_input(&input_red, &input_blue);

        Game::limit_speed(
            &mut self.rigid_body_set[self.ball_body_handle],
            BALL_TOP_SPEED,
        );

        self.physics_pipeline.step(
            &vector![0.0, 0.0],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );

        if self.goal_scored() && self.reset_timer <= 0 {
            self.start_reset_timer();
        }
    }
    fn parse_player_input(&mut self, input_red: &PlayerInput, input_blue: &PlayerInput) {
        let red_player_body = &mut self.rigid_body_set[self.red_player_body_handle];
        if input_red.up {
            red_player_body.apply_impulse(vector![0.0, -PLAYER_ACCELERATION], true);
        } else if input_red.down {
            red_player_body.apply_impulse(vector![0.0, PLAYER_ACCELERATION], true);
        }
        if input_red.left {
            red_player_body.apply_impulse(vector![-PLAYER_ACCELERATION, 0.0], true);
        } else if input_red.right {
            red_player_body.apply_impulse(vector![PLAYER_ACCELERATION, 0.0], true);
        }
        Game::limit_speed(red_player_body, PLAYER_TOP_SPEED);

        let blue_player_body = &mut self.rigid_body_set[self.blue_player_body_handle];
        if input_blue.up {
            blue_player_body.apply_impulse(vector![0.0, -PLAYER_ACCELERATION], true);
        } else if input_blue.down {
            blue_player_body.apply_impulse(vector![0.0, PLAYER_ACCELERATION], true);
        }
        if input_blue.left {
            blue_player_body.apply_impulse(vector![-PLAYER_ACCELERATION, 0.0], true);
        } else if input_blue.right {
            blue_player_body.apply_impulse(vector![PLAYER_ACCELERATION, 0.0], true);
        }
        Game::limit_speed(blue_player_body, PLAYER_TOP_SPEED);

        if input_red.shoot {
            if !self.red_last_tick_shot {
                self.shoot_ball(self.red_player_body_handle);
                self.red_last_tick_shot = true;
            }
        } else {
            self.red_last_tick_shot = false;
        }
        if input_blue.shoot {
            if !self.blue_last_tick_shot {
                self.shoot_ball(self.blue_player_body_handle);
                self.blue_last_tick_shot = true;
            }
        } else {
            self.blue_last_tick_shot = false;
        }
    }
    fn shoot_ball(&mut self, player_body_handle: RigidBodyHandle) {
        let player_body = &mut self.rigid_body_set[player_body_handle];
        let px = player_body.translation().x;
        let py = player_body.translation().y;

        let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
        let bx = ball_body.translation().x;
        let by = ball_body.translation().y;

        let dx = bx - px;
        let dy = by - py;
        let dist_sqr = dx * dx + dy * dy;
        if dist_sqr <= SHOOTING_DISTANCE * SHOOTING_DISTANCE {
            let angle = angle(px, py, bx, by);
            let x_speed = BALL_TOP_SPEED * (std::f32::consts::PI * (angle / 180.0)).cos();
            let y_speed = BALL_TOP_SPEED * (std::f32::consts::PI * (angle / 180.0)).sin();
            ball_body.set_linvel(vector![x_speed, y_speed], true);
        }
    }
    fn limit_speed(rigid_body: &mut RigidBody, top_speed: f32) {
        let x_speed = rigid_body.linvel().x;
        let y_speed = rigid_body.linvel().y;
        let speed = f32::sqrt(x_speed * x_speed + y_speed * y_speed);
        if speed > top_speed {
            let speed_normalized = rigid_body.linvel().normalize();
            rigid_body.set_linvel(
                vector![
                    speed_normalized.x * top_speed,
                    speed_normalized.y * top_speed
                ],
                true,
            );
        }
    }
    fn goal_scored(&mut self) -> bool {
        let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
        let x = ball_body.translation().x;
        if x < PITCH_LEFT_LINE {
            self.blue_scored = true;
            true
        } else if x > PITCH_RIGHT_LINE {
            self.red_scored = true;
            true
        } else {
            false
        }
    }
    fn start_reset_timer(&mut self) {
        self.reset_timer = RESET_TIME;
    }
    fn timer_tick(&mut self) {
        self.reset_timer -= 1;
        if self.reset_timer <= 0 {
            self.blue_scored = false;
            self.red_scored = false;
            self.reset_game();
        }
    }
    fn reset_game(&mut self) {
        let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
        ball_body.set_position(
            Isometry::new(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0], 0.0),
            false,
        );
        ball_body.set_linvel(vector![0.0, 0.0], false);

        let red_body = &mut self.rigid_body_set[self.red_player_body_handle];
        red_body.set_position(
            Isometry::new(
                vector![PITCH_LEFT_LINE + PLAYER_DIAMETER, STADIUM_HEIGHT / 2.0],
                0.0,
            ),
            false,
        );
        red_body.set_linvel(vector![0.0, 0.0], false);

        let blue_body = &mut self.rigid_body_set[self.blue_player_body_handle];
        blue_body.set_position(
            Isometry::new(
                vector![PITCH_RIGHT_LINE - PLAYER_DIAMETER, STADIUM_HEIGHT / 2.0],
                0.0,
            ),
            false,
        );
        blue_body.set_linvel(vector![0.0, 0.0], false);
    }
    pub fn get_player_entities(&self) -> JsValue {
        let v: Vec<Circle> = self
            .players
            .iter()
            .map(|player| player.to_circle(&self.rigid_body_set))
            .collect();
        JsValue::from_serde(&v).unwrap()
    }
    pub fn get_ball_entity(&self) -> JsValue {
        let brb = &self.rigid_body_set[self.ball_body_handle];
        let be = Circle::new(
            brb.translation().x,
            brb.translation().y,
            BALL_RADIUS,
            false,
            -1,
        );
        JsValue::from_serde(&be).unwrap()
    }
    pub fn get_edge_entities(&self) -> JsValue {
        JsValue::from_serde(&self.edges).unwrap()
    }
    pub fn get_goals_post_entities(&self) -> JsValue {
        JsValue::from_serde(&self.goals_posts).unwrap()
    }
    pub fn get_pitch_line_width(&self) -> f32 {
        PITCH_LINE_BREADTH
    }
    pub fn get_stadium_width(&self) -> f32 {
        STADIUM_WIDTH
    }
    pub fn get_stadium_height(&self) -> f32 {
        STADIUM_HEIGHT
    }
    pub fn get_goal_breadth(&self) -> f32 {
        GOAL_BREADTH
    }
    pub fn pitch_left_line(&self) -> f32 {
        PITCH_LEFT_LINE
    }
    pub fn pitch_right_line(&self) -> f32 {
        PITCH_RIGHT_LINE
    }
    pub fn pitch_top_line(&self) -> f32 {
        PITCH_TOP_LINE
    }
    pub fn pitch_bottom_line(&self) -> f32 {
        PITCH_BOTTOM_LINE
    }
    pub fn red_scored(&self) -> bool {
        self.red_scored
    }
    pub fn blue_scored(&self) -> bool {
        self.blue_scored
    }
}

// ==== game entities and Serde values ====

struct Player {
    rigid_body_handle: RigidBodyHandle,
    radius: f32,
    red: bool,
    number: i32,
}

impl Player {
    pub fn new(rigid_body_handle: RigidBodyHandle, radius: f32, red: bool, number: i32) -> Player {
        Player {
            rigid_body_handle,
            radius,
            red,
            number,
        }
    }
    pub fn to_circle(&self, rigid_body_set: &RigidBodySet) -> Circle {
        let rb = &rigid_body_set[self.rigid_body_handle];
        Circle::new(
            rb.translation().x,
            rb.translation().y,
            self.radius,
            self.red,
            self.number,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    red: bool,
    player_number: i32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32, red: bool, number: i32) -> Circle {
        Circle {
            x,
            y,
            radius,
            red,
            player_number: number,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Edge {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    white: bool,
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
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shoot: bool,
}
