use crate::constants::{
    BALL_GROUP, BALL_RADIUS, BALL_TOP_SPEED, GOAL_BREADTH, GOAL_DEPTH, GOAL_POSTS_GROUP,
    PITCH_BOTTOM_LINE, PITCH_HEIGHT, PITCH_LEFT_LINE, PITCH_LINES_GROUP, PITCH_LINE_BREADTH,
    PITCH_RIGHT_LINE, PITCH_TOP_LINE, PITCH_VERTICAL_LINE_HEIGHT, PITCH_WIDTH, PLAYERS_GROUP,
    PLAYER_ACCELERATION, PLAYER_DIAMETER, PLAYER_RADIUS, PLAYER_TOP_SPEED, RESET_TIME,
    SHOOTING_DISTANCE, STADIUM_HEIGHT, STADIUM_WALLS_GROUP, STADIUM_WIDTH,
};
use crate::utils::{Circle, Edge, Message, Player, PlayerInput};
use rapier2d::dynamics::{
    CCDSolver, IntegrationParameters, IslandManager, JointSet, RigidBody, RigidBodyBuilder,
    RigidBodyHandle, RigidBodySet,
};
use rapier2d::geometry::{
    BroadPhase, ColliderBuilder, ColliderSet, InteractionGroups, NarrowPhase,
};
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::prelude::*;
use rusty_games_library::{ConnectionType, NetworkManager, SessionId};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Game {
    players: Rc<RefCell<Vec<Player>>>,
    edges: Vec<Edge>,
    goal_posts: Vec<Circle>,
    ball_body_handle: RigidBodyHandle,
    red_scored: bool,
    blue_scored: bool,
    reset_timer: u32,

    // required by networking crate
    network_manager: NetworkManager,
    is_host: bool,

    // stuff required by physics engine
    rigid_body_set: Rc<RefCell<RigidBodySet>>,
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
        let ws_ip_port = "ws://ec2-3-71-106-87.eu-central-1.compute.amazonaws.com/ws";
        let network_manager = NetworkManager::new(ws_ip_port, session_id, ConnectionType::Stun)
            .expect("failed to create network manager");
        // let network_manager = NetworkManager::new(env!("WS_IP_PORT"), session_id, ConnectionType::Stun).expect("failed to create network manager");

        let rigid_body_set = Rc::new(RefCell::new(RigidBodySet::new()));
        let mut collider_set = ColliderSet::new();

        let edges = Game::create_pitch_lines(&mut collider_set);
        let goal_posts = Game::create_goals_posts(&mut collider_set);
        Game::create_stadium_walls(&mut collider_set);

        let players = Game::create_players(&mut rigid_body_set.borrow_mut(), &mut collider_set);
        let players = Rc::new(RefCell::new(players));

        let ball_body_handle =
            Game::create_ball(&mut rigid_body_set.borrow_mut(), &mut collider_set);

        Game {
            network_manager,
            is_host,
            players,
            edges,
            goal_posts,
            ball_body_handle,
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

    fn create_players(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Vec<Player> {
        let mut players = Vec::new();
        const COLLISION_GROUP: u32 =
            PLAYERS_GROUP | STADIUM_WALLS_GROUP | BALL_GROUP | GOAL_POSTS_GROUP;
        let mut create_player_closure = |x, y, red, number| {
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
            players.push(Player::new(player_body_handle, PLAYER_RADIUS, red, number));
        };

        create_player_closure(
            PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER,
            STADIUM_HEIGHT / 2.0,
            true,
            1,
        );
        create_player_closure(
            PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER,
            STADIUM_HEIGHT / 2.0,
            false,
            1,
        );

        players
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

    pub fn start(&mut self) {
        if self.is_host {
            self.start_as_host();
        } else {
            self.start_as_gamer();
        }
    }

    fn start_as_host(&mut self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            crate::utils::request_animation_frame(f.borrow().as_ref().unwrap());

            crate::tick_from_js();
            crate::host_send_state_from_js();
            crate::draw_from_js();
        }) as Box<dyn FnMut()>));

        let on_open_callback = move || {
            crate::utils::request_animation_frame(g.borrow().as_ref().unwrap());
        };

        let players = self.players.clone();
        let on_message_callback = move |message: String| {
            let input = serde_json::from_str::<PlayerInput>(&message).unwrap();
            players.borrow_mut()[1].set_input(input);
        };

        self.network_manager
            .start(on_open_callback, on_message_callback)
            .expect("network manager failed to start");
    }

    fn start_as_gamer(&mut self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let network_manager = self.network_manager.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            crate::utils::request_animation_frame(f.borrow().as_ref().unwrap());

            // on each frame, send input to host
            let message = serde_json::to_string::<PlayerInput>(
                &crate::get_player_input_from_js().into_serde().unwrap(),
            )
            .unwrap();
            network_manager.send_message(&message);

            crate::draw_from_js();
        }) as Box<dyn FnMut()>));

        let on_open_callback = move || {
            crate::utils::request_animation_frame(g.borrow().as_ref().unwrap());
        };

        let rigid_body_set_clone = self.rigid_body_set.clone();
        let red_handle = self.players.borrow()[0].rigid_body_handle;
        let blue_handle = self.players.borrow()[1].rigid_body_handle;
        let ball_handle = self.ball_body_handle;

        let on_message_callback = move |message: String| {
            let message = serde_json::from_str::<Message>(&message).unwrap();

            match message {
                Message::GameState {
                    red_x,
                    red_y,
                    blue_x,
                    blue_y,
                    ball_x,
                    ball_y,
                } => {
                    {
                        let red_body = &mut rigid_body_set_clone.borrow_mut()[red_handle];
                        red_body.set_position(Isometry::new(vector![red_x, red_y], 0.0), false);
                    }
                    {
                        let blue_body = &mut rigid_body_set_clone.borrow_mut()[blue_handle];
                        blue_body.set_position(Isometry::new(vector![blue_x, blue_y], 0.0), false);
                    }
                    {
                        let ball_body = &mut rigid_body_set_clone.borrow_mut()[ball_handle];
                        ball_body.set_position(Isometry::new(vector![ball_x, ball_y], 0.0), false);
                    }
                }
                Message::TeamScored {
                    did_red_scored: _,
                    red_current_score: _,
                    blue_current_score: _,
                } => todo!(),
                Message::GameEnded {
                    red_current_score: _,
                    blue_current_score: _,
                } => todo!(),
            };
        };

        self.network_manager
            .start(on_open_callback, on_message_callback)
            .expect("network manager failed to start");
    }

    pub fn host_send_state(&self) {
        let body_set = self.rigid_body_set.borrow();
        let red_pos = body_set[self.players.borrow()[0].rigid_body_handle].translation();
        let blue_pos = body_set[self.players.borrow()[1].rigid_body_handle].translation();
        let ball_pos = body_set[self.ball_body_handle].translation();

        let game_state = Message::GameState {
            red_x: red_pos.x,
            red_y: red_pos.y,
            blue_x: blue_pos.x,
            blue_y: blue_pos.y,
            ball_x: ball_pos.x,
            ball_y: ball_pos.y,
        };
        let game_state = serde_json::to_string(&game_state).unwrap();
        self.network_manager.send_message(&game_state);
    }

    pub fn tick(&mut self) {
        if self.reset_timer > 0 {
            self.timer_tick();
        }

        self.players.borrow_mut()[0]
            .set_input(crate::get_player_input_from_js().into_serde().unwrap());
        self.parse_input();

        Game::limit_speed(
            &mut self.rigid_body_set.borrow_mut()[self.ball_body_handle],
            BALL_TOP_SPEED,
        );

        self.physics_pipeline.step(
            &vector![0.0, 0.0],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set.borrow_mut(),
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );

        if self.goal_scored() {
            self.reset_timer = RESET_TIME;
        }
    }

    fn parse_input(&mut self) {
        let len = self.players.borrow().len();
        for i in 0..len {
            self.parse_player_input(i as usize);
        }
    }

    fn parse_player_input(&mut self, player_index: usize) {
        let player_last_tick_shot = self.players.borrow()[player_index].last_tick_shot;
        let input = self.players.borrow()[player_index].get_input().clone();
        let body_handle = self.players.borrow()[player_index].rigid_body_handle;

        if input.shoot && !player_last_tick_shot {
            self.shoot_ball(body_handle);
            self.players.borrow_mut()[player_index].set_last_tick_shot(true);
        } else {
            self.players.borrow_mut()[player_index].set_last_tick_shot(false);
        }

        let player_body = &mut self.rigid_body_set.borrow_mut()[body_handle];

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

        Game::limit_speed(player_body, PLAYER_TOP_SPEED);
    }

    fn shoot_ball(&mut self, player_body_handle: RigidBodyHandle) {
        let px;
        let py;
        {
            let player_body = &self.rigid_body_set.borrow()[player_body_handle];
            px = player_body.translation().x;
            py = player_body.translation().y;
        }

        let ball_body = &mut self.rigid_body_set.borrow_mut()[self.ball_body_handle];
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

    fn goal_scored(&mut self) -> bool {
        let ball_body = &mut self.rigid_body_set.borrow_mut()[self.ball_body_handle];
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

    fn timer_tick(&mut self) {
        self.reset_timer -= 1;
        self.blue_scored = false;
        self.red_scored = false;
        self.reset_game();
    }

    fn reset_game(&mut self) {
        {
            let ball_body = &mut self.rigid_body_set.borrow_mut()[self.ball_body_handle];
            ball_body.set_position(
                Isometry::new(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0], 0.0),
                false,
            );
            ball_body.set_linvel(vector![0.0, 0.0], false);
        }

        for player in self.players.borrow_mut().iter_mut() {
            player.reset_position(&mut self.rigid_body_set.borrow_mut(), 0.0, 0.0);
        }
    }

    pub fn get_player_entities(&self) -> JsValue {
        let v: Vec<Circle> = self
            .players
            .borrow()
            .iter()
            .map(|player| player.to_circle(&self.rigid_body_set.borrow()))
            .collect();
        JsValue::from_serde(&v).unwrap()
    }

    pub fn get_ball_entity(&self) -> JsValue {
        let brb = &self.rigid_body_set.borrow()[self.ball_body_handle];
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
        self.red_scored
    }

    pub fn get_blue_scored(&self) -> bool {
        self.blue_scored
    }
}
