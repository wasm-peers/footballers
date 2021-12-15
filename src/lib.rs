mod utils;

use rapier2d::{na::Vector2, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn alert(s: &str);
}

const PLAYER_DIAMETER: f32 = 30.0;
const PLAYER_RADIUS: f32 = PLAYER_DIAMETER / 2.0;
const PLAYER_ACCELERATION: f32 = 3000.0;
const BALL_RADIUS: f32 = 10.0;

const GOAL_BREADTH: f32 = 120.0;
const GOAL_DEPTH: f32 = 3.0 * BALL_RADIUS;
const PITCH_VERTICAL_LINE_HEIGHT: f32 = (PITCH_HEIGHT - GOAL_BREADTH) / 2.0;

const PITCH_WIDTH: f32 = 300.0;
const PITCH_HEIGHT: f32 = 530.0;
const PITCH_LINE_BREADTH: f32 = 3.0;
const PITCH_LEFT_LINE: f32 = 0.0 + 2.0 * PLAYER_DIAMETER;
const PITCH_RIGHT_LINE: f32 = PITCH_LEFT_LINE + PITCH_WIDTH;
const PITCH_TOP_LINE: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_BOTTOM_LINE: f32 = PITCH_TOP_LINE + PITCH_HEIGHT;
const STADIUM_WIDTH: f32 = 2.0 * PLAYER_DIAMETER + PITCH_WIDTH + 2.0 * PLAYER_DIAMETER;
const STADIUM_HEIGHT: f32 = 2.0 * PLAYER_DIAMETER + PITCH_HEIGHT;

// collision groups
const PITCH_LINES_GROUP: u32 = 0b_0000_0001;
const GOALS_GROUP: u32 = 0b_0000_0010;
const PLAYERS_GROUP: u32 = 0b_0000_0100;
const STADIUM_WALLS_GROUP: u32 = 0b_0000_1000;
const BALL_GROUP: u32 = 0b_0001_0000;

#[wasm_bindgen]
struct Game {
    players: Vec<Player>,
    edges: Vec<Edge>,
    goals: Vec<Circle>,
    player_body_handle: RigidBodyHandle,
    ball_body_handle: RigidBodyHandle,
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
    pub fn new() -> Game {
        let mut players: Vec<Player> = Vec::new();
        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();
        let mut edges = Vec::new();
        let mut collider_set: ColliderSet = ColliderSet::new();
        let mut goals = Vec::new();

        Game::create_pitch_lines(&mut collider_set, &mut edges);
        Game::create_goals(&mut collider_set, &mut goals);
        Game::create_stadium_walls(&mut collider_set);
        let player_body_handle =
            Game::create_players(&mut rigid_body_set, &mut collider_set, &mut players);
        let ball_body_handle = Game::create_ball(&mut rigid_body_set, &mut collider_set);
        Game {
            players,
            edges,
            goals,
            player_body_handle,
            ball_body_handle,
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
        create_line_closure(PITCH_LINE_BREADTH, PITCH_VERTICAL_LINE_HEIGHT, PITCH_LEFT_LINE, (STADIUM_HEIGHT - GOAL_BREADTH - PITCH_VERTICAL_LINE_HEIGHT) / 2.0, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        // left lower pitch line
        create_line_closure(PITCH_LINE_BREADTH, PITCH_VERTICAL_LINE_HEIGHT, PITCH_LEFT_LINE, (STADIUM_HEIGHT + GOAL_BREADTH + PITCH_VERTICAL_LINE_HEIGHT) / 2.0, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        // left goal
        create_line_closure(PITCH_LINE_BREADTH, GOAL_BREADTH, PITCH_LEFT_LINE - GOAL_DEPTH, STADIUM_HEIGHT / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        create_line_closure(GOAL_DEPTH, PITCH_LINE_BREADTH, PITCH_LEFT_LINE - GOAL_DEPTH / 2.0, (STADIUM_HEIGHT - GOAL_BREADTH) / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        create_line_closure(GOAL_DEPTH, PITCH_LINE_BREADTH, PITCH_LEFT_LINE - GOAL_DEPTH / 2.0, (STADIUM_HEIGHT + GOAL_BREADTH) / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);

        // right higher pitch line
        create_line_closure(PITCH_LINE_BREADTH, PITCH_VERTICAL_LINE_HEIGHT, PITCH_RIGHT_LINE, (STADIUM_HEIGHT - GOAL_BREADTH - PITCH_VERTICAL_LINE_HEIGHT) / 2.0, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        // right lower pitch line
        create_line_closure(PITCH_LINE_BREADTH, PITCH_VERTICAL_LINE_HEIGHT, PITCH_RIGHT_LINE, (STADIUM_HEIGHT + GOAL_BREADTH + PITCH_VERTICAL_LINE_HEIGHT) / 2.0, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        // right goal
        create_line_closure(PITCH_LINE_BREADTH, GOAL_BREADTH, PITCH_RIGHT_LINE + GOAL_DEPTH, STADIUM_HEIGHT / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        create_line_closure(GOAL_DEPTH, PITCH_LINE_BREADTH, PITCH_RIGHT_LINE + GOAL_DEPTH / 2.0, (STADIUM_HEIGHT - GOAL_BREADTH) / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
        create_line_closure(GOAL_DEPTH, PITCH_LINE_BREADTH, PITCH_RIGHT_LINE + GOAL_DEPTH / 2.0, (STADIUM_HEIGHT + GOAL_BREADTH) / 2.0, false, PITCH_LINES_GROUP, PITCH_LINES_GROUP);

        // top pitch line`
        create_line_closure(PITCH_WIDTH, PITCH_LINE_BREADTH, STADIUM_WIDTH / 2.0, PITCH_TOP_LINE, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);

        // bottom pitch line
        create_line_closure(PITCH_WIDTH, PITCH_LINE_BREADTH, STADIUM_WIDTH / 2.0, PITCH_BOTTOM_LINE, true, PITCH_LINES_GROUP, PITCH_LINES_GROUP);
    }
    fn create_goals(collider_set: &mut ColliderSet, edges: &mut Vec<Circle>) {
        // left red goal
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(GOALS_GROUP, GOALS_GROUP))
            .translation(vector![
                PITCH_LEFT_LINE,
                PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 - GOAL_BREADTH / 2.0
            ])
            .build();
        edges.push(Circle::new(
            ball_collider.translation().x,
            ball_collider.translation().y,
            BALL_RADIUS,
            true,
            -1,
        ));
        collider_set.insert(ball_collider);

        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(GOALS_GROUP, GOALS_GROUP))
            .translation(vector![
                PITCH_LEFT_LINE,
                PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 + GOAL_BREADTH / 2.0
            ])
            .build();
        edges.push(Circle::new(
            ball_collider.translation().x,
            ball_collider.translation().y,
            BALL_RADIUS,
            true,
            -1,
        ));
        collider_set.insert(ball_collider);

        // right blue goal
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(GOALS_GROUP, GOALS_GROUP))
            .translation(vector![
                PITCH_RIGHT_LINE,
                PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 - GOAL_BREADTH / 2.0
            ])
            .build();
        edges.push(Circle::new(
            ball_collider.translation().x,
            ball_collider.translation().y,
            BALL_RADIUS,
            false,
            -1,
        ));
        collider_set.insert(ball_collider);

        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(GOALS_GROUP, GOALS_GROUP))
            .translation(vector![
                PITCH_RIGHT_LINE,
                PITCH_TOP_LINE + PITCH_HEIGHT / 2.0 + GOAL_BREADTH / 2.0
            ])
            .build();
        edges.push(Circle::new(
            ball_collider.translation().x,
            ball_collider.translation().y,
            BALL_RADIUS,
            false,
            -1,
        ));
        collider_set.insert(ball_collider);
    }
    fn create_stadium_walls(collider_set: &mut ColliderSet) {
        // left stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(0.0, STADIUM_HEIGHT)
            .collision_groups(InteractionGroups::new(
                STADIUM_WALLS_GROUP,
                STADIUM_WALLS_GROUP,
            ))
            .translation(vector![0.0, STADIUM_HEIGHT / 2.0])
            .build();
        collider_set.insert(cuboid_collider);

        // right stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(0.0, STADIUM_HEIGHT)
            .collision_groups(InteractionGroups::new(
                STADIUM_WALLS_GROUP,
                STADIUM_WALLS_GROUP,
            ))
            .translation(vector![STADIUM_WIDTH, STADIUM_HEIGHT / 2.0])
            .build();
        collider_set.insert(cuboid_collider);

        // top stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(STADIUM_WIDTH, 0.0)
            .collision_groups(InteractionGroups::new(
                STADIUM_WALLS_GROUP,
                STADIUM_WALLS_GROUP,
            ))
            .translation(vector![STADIUM_WIDTH / 2.0, 0.0])
            .build();
        collider_set.insert(cuboid_collider);

        // bottom stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(STADIUM_WIDTH, 0.0)
            .collision_groups(InteractionGroups::new(
                STADIUM_WALLS_GROUP,
                STADIUM_WALLS_GROUP,
            ))
            .translation(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT])
            .build();
        collider_set.insert(cuboid_collider);
    }
    fn create_players(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        players: &mut Vec<Player>,
    ) -> RigidBodyHandle {
        const COLLISION_GROUP: u32 = PLAYERS_GROUP | STADIUM_WALLS_GROUP | BALL_GROUP | GOALS_GROUP;

        // create red player
        let player_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.5)
            .translation(vector![
                PITCH_LEFT_LINE + 2.0 * PLAYER_DIAMETER,
                STADIUM_HEIGHT / 2.0
            ])
            .build();
        let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
            .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
            .restitution(0.7)
            .build();
        let player_body_handle: RigidBodyHandle = rigid_body_set.insert(player_rigid_body);
        collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
        players.push(Player::new(player_body_handle, PLAYER_RADIUS, true, 1));

        // create blue zombie players
        for i in 0..=1 {
            let player_rigid_body = RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .translation(vector![
                    PITCH_RIGHT_LINE - 2.0 * PLAYER_DIAMETER as f32,
                    STADIUM_HEIGHT / 2.0 - PLAYER_DIAMETER + 2.0 * PLAYER_DIAMETER * i as f32
                ])
                .build();
            let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
                .collision_groups(InteractionGroups::new(COLLISION_GROUP, COLLISION_GROUP))
                .restitution(0.7)
                .build();
            let player_body_handle = rigid_body_set.insert(player_rigid_body);
            collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
            players.push(Player::new(player_body_handle, PLAYER_RADIUS, false, i));
        }

        return player_body_handle;
    }
    fn create_ball(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> RigidBodyHandle {
        const COLLISION_GROUP: u32 = BALL_GROUP | PLAYERS_GROUP | PITCH_LINES_GROUP | GOALS_GROUP;

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
    pub fn tick(&mut self, val: &JsValue) {
        let input: PlayerInput = val.into_serde().unwrap();
        self.parse_player_input(&input);

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
    fn parse_player_input(&mut self, input: &PlayerInput) {
        let player_body = &mut self.rigid_body_set[self.player_body_handle];

        if input.shoot {}
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
    }
    pub fn get_player_entities(&self) -> JsValue {
        let v: Vec<Circle> = self
            .players
            .iter()
            .map(|player| player.create_entity(&self.rigid_body_set))
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
    pub fn get_goal_entities(&self) -> JsValue {
        JsValue::from_serde(&self.goals).unwrap()
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
}

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
    pub fn create_entity(&self, rigid_body_set: &RigidBodySet) -> Circle {
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
    white: bool
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
