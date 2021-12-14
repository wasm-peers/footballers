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
const PITCH_WIDTH: f32 = 300.0;
const PITCH_HEIGHT: f32 = 530.0;
const PITCH_LINE_WIDTH: f32 = 5.0;
const PITCH_LEFT_WALL: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_RIGHT_WALL: f32 = PITCH_LEFT_WALL + PITCH_WIDTH;
const PITCH_TOP_WALL: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_BOTTOM_WALL: f32 = PITCH_TOP_WALL + PITCH_HEIGHT;
const STADIUM_WIDTH: f32 = 2.0 * PLAYER_DIAMETER + PITCH_WIDTH;
const STADIUM_HEIGHT: f32 = 2.0 * PLAYER_DIAMETER + PITCH_HEIGHT;

const BALL_RADIUS: f32 = 10.0;

// collision groups
const LINES_GROUP: u32 = 0b_0000_0001;
const PLAYER_GROUP: u32 = 0b_0000_0100;
const STADIUM_GROUP: u32 = 0b_0000_1000;
const BALL_GROUP: u32 = 0b_0001_0000;

#[wasm_bindgen]
struct Game {
    players: Vec<Player>,
    walls: Vec<WallEntity>,
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
        let mut walls = Vec::new();
        let mut collider_set: ColliderSet = ColliderSet::new();

        Game::create_pitch_lines(&mut collider_set, &mut walls);
        Game::create_stadium_walls(&mut collider_set, &mut walls);
        let player_body_handle =
            Game::create_players(&mut rigid_body_set, &mut collider_set, &mut players);
        let ball_body_handle = Game::create_ball(&mut rigid_body_set, &mut collider_set);

        Game {
            players,
            walls,
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
    fn create_pitch_lines(collider_set: &mut ColliderSet, walls: &mut Vec<WallEntity>) {
        // left pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_LINE_WIDTH / 2.0, PITCH_HEIGHT / 2.0)
            .collision_groups(InteractionGroups::new(LINES_GROUP, LINES_GROUP))
            .translation(vector![
                PITCH_LEFT_WALL,
                PITCH_TOP_WALL + PITCH_HEIGHT / 2.0
            ])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            PITCH_LINE_WIDTH,
            PITCH_HEIGHT,
        ));
        collider_set.insert(cuboid_collider);

        // right pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_LINE_WIDTH / 2.0, PITCH_HEIGHT / 2.0)
            .collision_groups(InteractionGroups::new(LINES_GROUP, LINES_GROUP))
            .translation(vector![
                PITCH_RIGHT_WALL,
                PITCH_TOP_WALL + PITCH_HEIGHT / 2.0
            ])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            PITCH_LINE_WIDTH,
            PITCH_HEIGHT,
        ));
        collider_set.insert(cuboid_collider);

        // top pitch line`
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_WIDTH / 2.0, PITCH_LINE_WIDTH / 2.0)
            .collision_groups(InteractionGroups::new(LINES_GROUP, LINES_GROUP))
            .translation(vector![PITCH_LEFT_WALL + PITCH_WIDTH / 2.0, PITCH_TOP_WALL])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            PITCH_WIDTH,
            PITCH_LINE_WIDTH,
        ));
        collider_set.insert(cuboid_collider);

        // bottom pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_WIDTH / 2.0, PITCH_LINE_WIDTH / 2.0)
            .collision_groups(InteractionGroups::new(LINES_GROUP, LINES_GROUP))
            .translation(vector![
                PITCH_LEFT_WALL + PITCH_WIDTH / 2.0,
                PITCH_BOTTOM_WALL
            ])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            PITCH_WIDTH,
            PITCH_LINE_WIDTH,
        ));
        collider_set.insert(cuboid_collider);
    }
    fn create_stadium_walls(collider_set: &mut ColliderSet, walls: &mut Vec<WallEntity>) {
        // left stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(0.0, STADIUM_HEIGHT)
            .collision_groups(InteractionGroups::new(STADIUM_GROUP, STADIUM_GROUP))
            .translation(vector![0.0, STADIUM_HEIGHT / 2.0])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            0.0,
            STADIUM_HEIGHT,
        ));
        collider_set.insert(cuboid_collider);

        // right stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(0.0, STADIUM_HEIGHT)
            .collision_groups(InteractionGroups::new(STADIUM_GROUP, STADIUM_GROUP))
            .translation(vector![STADIUM_WIDTH, STADIUM_HEIGHT / 2.0])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            0.0,
            STADIUM_HEIGHT,
        ));
        collider_set.insert(cuboid_collider);

        // top stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(STADIUM_WIDTH, 0.0)
            .collision_groups(InteractionGroups::new(STADIUM_GROUP, STADIUM_GROUP))
            .translation(vector![STADIUM_WIDTH / 2.0, 0.0])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            STADIUM_WIDTH,
            0.0,
        ));
        collider_set.insert(cuboid_collider);

        // bottom stadium wall
        let cuboid_collider = ColliderBuilder::cuboid(STADIUM_WIDTH, 0.0)
            .collision_groups(InteractionGroups::new(STADIUM_GROUP, STADIUM_GROUP))
            .translation(vector![STADIUM_WIDTH / 2.0, STADIUM_HEIGHT])
            .build();
        walls.push(WallEntity::new(
            cuboid_collider.translation().x,
            cuboid_collider.translation().y,
            STADIUM_WIDTH,
            0.0,
        ));
        collider_set.insert(cuboid_collider);
    }
    fn create_players(
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        players: &mut Vec<Player>,
    ) -> RigidBodyHandle {
        // create red player
        let player_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.5)
            .translation(vector![200.0, PLAYER_DIAMETER + 3.0 * PLAYER_DIAMETER])
            .build();
        let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
            .collision_groups(InteractionGroups::new(
                PLAYER_GROUP | STADIUM_GROUP | BALL_GROUP,
                PLAYER_GROUP | STADIUM_GROUP | BALL_GROUP,
            ))
            .restitution(0.7)
            .build();
        let player_body_handle: RigidBodyHandle = rigid_body_set.insert(player_rigid_body);
        collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
        players.push(Player::new(player_body_handle, PLAYER_RADIUS, true, 1));

        // create blue zombie players
        for i in 1..=2 {
            let player_rigid_body = RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .translation(vector![
                    2.0 * PLAYER_DIAMETER * i as f32,
                    2.0 * PLAYER_DIAMETER * i as f32
                ])
                .build();
            let player_collider = ColliderBuilder::ball(PLAYER_RADIUS)
                .collision_groups(InteractionGroups::new(
                    PLAYER_GROUP | STADIUM_GROUP | BALL_GROUP,
                    PLAYER_GROUP | STADIUM_GROUP | BALL_GROUP,
                ))
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
        let ball_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.5)
            .translation(vector![250.0, PLAYER_DIAMETER + 3.0 * PLAYER_DIAMETER])
            .build();
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS)
            .collision_groups(InteractionGroups::new(
                BALL_GROUP | PLAYER_GROUP | LINES_GROUP,
                BALL_GROUP | PLAYER_GROUP | LINES_GROUP,
            ))
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
        let v: Vec<BallEntity> = self
            .players
            .iter()
            .map(|player| player.create_entity(&self.rigid_body_set))
            .collect();
        JsValue::from_serde(&v).unwrap()
    }
    pub fn get_ball_entity(&self) -> JsValue {
        let brb = &self.rigid_body_set[self.ball_body_handle];
        let be = BallEntity::new(
            brb.translation().x,
            brb.translation().y,
            BALL_RADIUS,
            false,
            -1,
        );
        JsValue::from_serde(&be).unwrap()
    }
    pub fn get_wall_entities(&self) -> JsValue {
        JsValue::from_serde(&self.walls).unwrap()
    }
    pub fn get_pitch_line_width(&self) -> f32 {
        PITCH_LINE_WIDTH
    }
    pub fn get_stadium_width(&self) -> f32 {
        STADIUM_WIDTH
    }
    pub fn get_stadium_height(&self) -> f32 {
        STADIUM_HEIGHT
    }
    pub fn pitch_left_wall(&self) -> f32 {
        PITCH_LEFT_WALL
    }
    pub fn pitch_right_wall(&self) -> f32 {
        PITCH_RIGHT_WALL
    }
    pub fn pitch_top_wall(&self) -> f32 {
        PITCH_TOP_WALL
    }
    pub fn pitch_bottom_wall(&self) -> f32 {
        PITCH_BOTTOM_WALL
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
    pub fn create_entity(&self, rigid_body_set: &RigidBodySet) -> BallEntity {
        let rb = &rigid_body_set[self.rigid_body_handle];
        BallEntity::new(
            rb.translation().x,
            rb.translation().y,
            self.radius,
            self.red,
            self.number,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct BallEntity {
    x: f32,
    y: f32,
    radius: f32,
    red: bool,
    number: i32,
}

impl BallEntity {
    pub fn new(x: f32, y: f32, radius: f32, red: bool, number: i32) -> BallEntity {
        BallEntity {
            x,
            y,
            radius,
            red,
            number,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WallEntity {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl WallEntity {
    pub fn new(x_center: f32, y_center: f32, width: f32, height: f32) -> WallEntity {
        WallEntity {
            x: x_center - width / 2.0,
            y: y_center - height / 2.0,
            width,
            height,
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
