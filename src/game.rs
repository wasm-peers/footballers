use crate::constants::{
    BALL_GROUP, BALL_RADIUS, BALL_TOP_SPEED, GOAL_BREADTH, GOAL_DEPTH, GOAL_POSTS_GROUP, MAX_GOALS,
    PITCH_BOTTOM_LINE, PITCH_HEIGHT, PITCH_LEFT_LINE, PITCH_LINES_GROUP, PITCH_LINE_BREADTH,
    PITCH_RIGHT_LINE, PITCH_TOP_LINE, PITCH_VERTICAL_LINE_HEIGHT, PITCH_WIDTH, PLAYERS_GROUP,
    PLAYER_ACCELERATION, PLAYER_DIAMETER, PLAYER_RADIUS, PLAYER_TOP_SPEED, RESET_TIME,
    SHOOTING_DISTANCE, STADIUM_HEIGHT, STADIUM_WALLS_GROUP, STADIUM_WIDTH,
};
use crate::utils::{Arbiter, Circle, Edge, Message, Player, PlayerInput, PlayerPosition, Score};
use rapier2d::dynamics::{
    CCDSolver, IntegrationParameters, IslandManager, JointSet, RigidBody, RigidBodyBuilder,
    RigidBodyHandle, RigidBodySet,
};
use rapier2d::geometry::{
    BroadPhase, ColliderBuilder, ColliderSet, InteractionGroups, NarrowPhase,
};
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::prelude::*;
use rusty_games_library::one_to_many::MiniServer;
use rusty_games_library::{ConnectionType, SessionId, UserId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct HostGame {
    inner: Rc<RefCell<HostGameInner>>,
}

#[wasm_bindgen]
impl HostGame {
    pub fn new(session_id: String) -> HostGame {
        HostGame {
            inner: Rc::new(RefCell::new(HostGameInner::new(session_id))),
        }
    }

    pub fn start(&mut self) {
        let host_game = self.inner.clone();
        let on_open_callback = move |user_id| {
            if !host_game.borrow().game_started {
                let g = Closure::wrap(Box::new(move || {
                    crate::check_timer_from_js();
                    crate::tick_from_js();
                    crate::host_send_state_from_js();
                    crate::draw_from_js();
                }) as Box<dyn FnMut()>);
                crate::utils::set_interval_with_callback(&g);
                g.forget();
                host_game.borrow_mut().game_started = true;
            }

            let red_players_count = 1 + host_game
                .borrow()
                .players
                .values()
                .filter(|player| player.red)
                .count();
            let blue_players_count = host_game
                .borrow()
                .players
                .values()
                .filter(|player| !player.red)
                .count();
            if red_players_count < blue_players_count {
                host_game.borrow_mut().players.insert(
                    user_id,
                    HostGameInner::create_player(
                        &mut host_game.borrow_mut().rigid_body_set,
                        &mut host_game.borrow_mut().collider_set,
                        PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER,
                        STADIUM_HEIGHT / 2.0,
                        true,
                        red_players_count + 1,
                    ),
                );
            } else {
                host_game.borrow_mut().players.insert(
                    user_id,
                    HostGameInner::create_player(
                        &mut host_game.borrow_mut().rigid_body_set,
                        &mut host_game.borrow_mut().collider_set,
                        PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER,
                        STADIUM_HEIGHT / 2.0,
                        false,
                        blue_players_count + 1,
                    ),
                );
            }
        };

        let host_game = self.inner.clone();
        let on_message_callback = move |user_id, message: String| {
            let input = serde_json::from_str::<PlayerInput>(&message).unwrap();
            host_game
                .borrow_mut()
                .players
                .get_mut(&user_id)
                .expect("no player instance for this user_id")
                .set_input(input);
        };

        self.inner
            .borrow_mut()
            .mini_server
            .start(on_open_callback, on_message_callback)
            .expect("network manager failed to start");
    }
}

struct HostGameInner {
    host_player: Player,
    players: HashMap<UserId, Player>,
    edges: Vec<Edge>,
    goal_posts: Vec<Circle>,
    ball_body_handle: RigidBodyHandle,
    arbiter: Arbiter,

    // required by networking crate
    mini_server: MiniServer,
    game_started: bool,

    // stuff required by physics engine
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

impl HostGameInner {
    pub fn new(session_id: String) -> HostGameInner {
        // let connection_type = ConnectionType::StunAndTurn {
        //     stun_urls: env!("STUN_SERVER_URLS").to_string(),
        //     turn_urls: env!("TURN_SERVER_URLS").to_string(),
        //     username: env!("TURN_SERVER_USERNAME").to_string(),
        //     credential: env!("TURN_SERVER_CREDENTIAL").to_string(),
        // };
        let connection_type = ConnectionType::Local;
        let session_id = SessionId::new(session_id);
        let mini_server = MiniServer::new(
            concat!(env!("SIGNALING_SERVER_URL"), "/one-to-one"),
            session_id,
            connection_type,
        )
        .expect("failed to create network manager");

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let edges = HostGameInner::create_pitch_lines(&mut collider_set);
        let goal_posts = HostGameInner::create_goals_posts(&mut collider_set);
        HostGameInner::create_stadium_walls(&mut collider_set);

        // create_player_closure(
        //     PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER,
        //     STADIUM_HEIGHT / 2.0,
        //     false,
        //     1,
        // );
        let host_player = HostGameInner::create_player(
            &mut rigid_body_set,
            &mut collider_set,
            PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER,
            STADIUM_HEIGHT / 2.0,
            true,
            1,
        );

        let ball_body_handle = HostGameInner::create_ball(&mut rigid_body_set, &mut collider_set);

        HostGameInner {
            mini_server,
            game_started: false,
            host_player,
            players: HashMap::new(),
            edges,
            goal_posts,
            ball_body_handle,
            arbiter: Arbiter::new(),
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

    fn create_pitch_lines(collider_set: &mut ColliderSet) -> Vec<Edge> {
        let mut edges = Vec::new();
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

        edges
    }

    fn create_goals_posts(collider_set: &mut ColliderSet) -> Vec<Circle> {
        let mut goal_posts = Vec::new();

        let mut create_post_closure = |x, y, red| {
            let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
                .collision_groups(InteractionGroups::new(GOAL_POSTS_GROUP, GOAL_POSTS_GROUP))
                .translation(vector![x, y])
                .build();
            goal_posts.push(Circle::new(
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

        goal_posts
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

    fn create_player(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        x: f32,
        y: f32,
        is_red: bool,
        number: usize,
    ) -> Player {
        const COLLISION_GROUP: u32 =
            PLAYERS_GROUP | STADIUM_WALLS_GROUP | BALL_GROUP | GOAL_POSTS_GROUP;
        let player_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(1.0)
            .translation(vector![x, y])
            .build();
        let player_rigid_body = Rc::new(RefCell::new(player_rigid_body));
        let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
            .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
            .restitution(0.7)
            .build();
        let player_body_handle: RigidBodyHandle =
            rigid_body_set.insert(player_rigid_body.borrow().to_owned());
        collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
        Player::new(player_body_handle, PLAYER_RADIUS, is_red, number)
    }

    fn create_ball(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> RigidBodyHandle {
        const COLLISION_GROUP: u32 =
            BALL_GROUP | PLAYERS_GROUP | PITCH_LINES_GROUP | GOAL_POSTS_GROUP;

        let ball_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.3)
            .translation(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0])
            .build();
        let ball_rigid_body = Rc::new(RefCell::new(ball_rigid_body));
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .density(0.5)
            .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
            .restitution(0.7)
            .build();
        let ball_body_handle: RigidBodyHandle =
            rigid_body_set.insert(ball_rigid_body.borrow().to_owned());
        collider_set.insert_with_parent(ball_collider, ball_body_handle, rigid_body_set);

        ball_body_handle
    }

    pub fn host_send_state(&mut self) {
        let body_set = &self.rigid_body_set;

        let game_state = if self.arbiter.send_score_message {
            self.arbiter.send_score_message = false;
            Message::GoalScored {
                did_red_score: self.arbiter.red_scored,
            }
        } else {
            let players = self
                .players
                .values()
                .map(|player| {
                    let player_pos = body_set[player.rigid_body_handle].translation();
                    PlayerPosition {
                        x: player_pos.x,
                        y: player_pos.y,
                        red: player.red,
                    }
                })
                .collect();
            let ball_pos = body_set[self.ball_body_handle].translation();
            Message::GameState {
                players,
                ball_x: ball_pos.x,
                ball_y: ball_pos.y,
            }
        };
        let game_state = serde_json::to_string(&game_state).unwrap();

        self.mini_server.send_message_to_all(&game_state);
    }

    pub fn tick(&mut self) {
        self.host_player
            .set_input(crate::get_player_input_from_js().into_serde().unwrap());
        self.parse_input();

        HostGameInner::limit_speed(
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
    }

    fn parse_input(&mut self) {
        for player in self.players.values_mut() {
            let player_last_tick_shot = player.last_tick_shot;
            let input = player.get_input().clone();
            let body_handle = player.rigid_body_handle;

            if input.shoot {
                if !player_last_tick_shot {
                    let px;
                    let py;
                    {
                        let player_body = &self.rigid_body_set[body_handle];
                        px = player_body.translation().x;
                        py = player_body.translation().y;
                    }

                    let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
                    let bx = ball_body.translation().x;
                    let by = ball_body.translation().y;

                    let dx = bx - px;
                    let dy = by - py;
                    let dist_sqr = dx * dx + dy * dy;
                    if dist_sqr <= SHOOTING_DISTANCE * SHOOTING_DISTANCE {
                        let angle = crate::utils::angle(px, py, bx, by);
                        let x_speed =
                            BALL_TOP_SPEED * (std::f32::consts::PI * (angle / 180.0)).cos();
                        let y_speed =
                            BALL_TOP_SPEED * (std::f32::consts::PI * (angle / 180.0)).sin();
                        ball_body.set_linvel(vector![x_speed, y_speed], true);
                    }
                    player.set_last_tick_shot(true);
                }
            } else {
                player.set_last_tick_shot(false);
            }

            let player_body = &mut self.rigid_body_set[body_handle];

            if input.up {
                player_body.apply_impulse(vector![0.0, -PLAYER_ACCELERATION], true);
            } else if input.down {
                player_body.apply_impulse(vector![0.0, PLAYER_ACCELERATION], true);
            }

            if input.left {
                player_body.apply_impulse(vector![-PLAYER_ACCELERATION, 0.0], true);
            } else if input.right {
                player_body.apply_impulse(vector![PLAYER_ACCELERATION, 0.0], true);
            }

            HostGameInner::limit_speed(player_body, PLAYER_TOP_SPEED);
        }
    }

    fn shoot_ball(&mut self, player_body_handle: RigidBodyHandle) {
        let px;
        let py;
        {
            let player_body = &self.rigid_body_set[player_body_handle];
            px = player_body.translation().x;
            py = player_body.translation().y;
        }

        let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
        let bx = ball_body.translation().x;
        let by = ball_body.translation().y;

        let dx = bx - px;
        let dy = by - py;
        let dist_sqr = dx * dx + dy * dy;
        if dist_sqr <= SHOOTING_DISTANCE * SHOOTING_DISTANCE {
            let angle = crate::utils::angle(px, py, bx, by);
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

    pub fn check_timer(&mut self) {
        if self.arbiter.game_ended {
            return;
        }
        if self.arbiter.reset_timer > 0 {
            self.timer_tick();
        } else if self.goal_scored() {
            self.arbiter.reset_timer = RESET_TIME;
        }
    }

    fn goal_scored(&mut self) -> bool {
        let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
        let x = ball_body.translation().x;
        if x < PITCH_LEFT_LINE {
            self.arbiter.set_blue_scored();
            true
        } else if x > PITCH_RIGHT_LINE {
            self.arbiter.set_red_scored();
            true
        } else {
            false
        }
    }

    fn timer_tick(&mut self) {
        self.arbiter.reset_timer -= 1;
        if self.arbiter.reset_timer == 0 {
            self.arbiter.reset_who_scored();
            self.check_ending();
            self.reset_game();
        }
    }

    fn check_ending(&mut self) {
        if self.arbiter.red_score == MAX_GOALS || self.arbiter.blue_score == MAX_GOALS {
            self.arbiter.game_ended = true;
        }
    }

    fn reset_game(&mut self) {
        {
            let ball_body = &mut self.rigid_body_set[self.ball_body_handle];
            ball_body.set_position(
                Isometry::new(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0], 0.0),
                false,
            );
            ball_body.set_linvel(vector![0.0, 0.0], false);
        }

        for player in self.players.values_mut() {
            player.reset_position(&mut self.rigid_body_set, 0.0, 0.0);
        }
    }

    pub fn get_player_entities(&self) -> JsValue {
        let v: Vec<Circle> = self
            .players
            .values()
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

    pub fn get_goal_posts_entities(&self) -> JsValue {
        JsValue::from_serde(&self.goal_posts).unwrap()
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

    pub fn get_pitch_left_line(&self) -> f32 {
        PITCH_LEFT_LINE
    }

    pub fn get_pitch_right_line(&self) -> f32 {
        PITCH_RIGHT_LINE
    }

    pub fn get_pitch_top_line(&self) -> f32 {
        PITCH_TOP_LINE
    }

    pub fn get_pitch_bottom_line(&self) -> f32 {
        PITCH_BOTTOM_LINE
    }

    pub fn get_red_scored(&self) -> bool {
        self.arbiter.red_scored
    }

    pub fn get_blue_scored(&self) -> bool {
        self.arbiter.blue_scored
    }

    pub fn get_score(&self) -> JsValue {
        let score = Score::new(self.arbiter.red_score, self.arbiter.blue_score);
        JsValue::from_serde(&score).unwrap()
    }

    pub fn get_game_ended(&self) -> bool {
        self.arbiter.game_ended
    }
}
