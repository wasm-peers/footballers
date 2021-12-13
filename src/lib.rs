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
const PITCH_WIDTH: f32 = 300.0;
const PITCH_HEIGHT: f32 = 530.0;
const PITCH_LINE_BREADTH: f32 = 2.0;
const PITCH_LEFT_WALL: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_RIGHT_WALL: f32 = PITCH_LEFT_WALL + PITCH_WIDTH;
const PITCH_TOP_WALL: f32 = 0.0 + PLAYER_DIAMETER;
const PITCH_BOTTOM_WALL: f32 = PITCH_TOP_WALL + PITCH_HEIGHT;
const STADIUM_LEFT_WALL: f32 = 0.0;
const STADIUM_RIGHT_WALL: f32 = PITCH_RIGHT_WALL + PLAYER_DIAMETER;
const STADIUM_TOP_WALL: f32 = 0.0;
const STADIUM_BOTTOM_WALL: f32 = PITCH_BOTTOM_WALL + PLAYER_DIAMETER;

#[wasm_bindgen]
struct Game {
    players: Vec<Player>,
    walls: Vec<WallEntity>,
    player_body_handle: RigidBodyHandle,
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

        Game::create_stadium(&mut collider_set, &mut walls);

        // create red player
        let player_rigid_body = RigidBodyBuilder::new_dynamic()
            .linear_damping(0.5)
            .translation(vector![200.0, PLAYER_DIAMETER + 3.0 * PLAYER_DIAMETER])
            .build();
        let player_collider = ColliderBuilder::ball(PLAYER_DIAMETER).restitution(0.7).build();
        let player_body_handle: RigidBodyHandle = rigid_body_set.insert(player_rigid_body);
        collider_set.insert_with_parent(player_collider, player_body_handle, &mut rigid_body_set);
        players.push(Player::new(player_body_handle, PLAYER_DIAMETER, true, 1));

        // create blue zombie players
        for i in 1..=4 {
            let ball_rigid_body = RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .translation(vector![2.0 * PLAYER_DIAMETER * i as f32, 2.0 * PLAYER_DIAMETER * i as f32])
                .build();
            let ball_collider = ColliderBuilder::ball(PLAYER_DIAMETER).restitution(0.7).build();
            let ball_body_handle = rigid_body_set.insert(ball_rigid_body);
            collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);
            players.push(Player::new(ball_body_handle, PLAYER_DIAMETER, false, i));
        }

        Game {
            players,
            walls,
            player_body_handle,
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
    fn create_stadium(collider_set: &mut ColliderSet, walls: &mut Vec<WallEntity>) {

        // left pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_LINE_BREADTH / 2.0, PITCH_HEIGHT / 2.0)
            .translation(vector![PITCH_LEFT_WALL, PITCH_TOP_WALL + PITCH_HEIGHT / 2.0])
            .build();
        walls.push(WallEntity::new(cuboid_collider.translation().x, cuboid_collider.translation().y, PITCH_LINE_BREADTH, PITCH_HEIGHT));
        collider_set.insert(cuboid_collider);

        // right pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_LINE_BREADTH / 2.0, PITCH_HEIGHT / 2.0)
            .translation(vector![PITCH_RIGHT_WALL, PITCH_TOP_WALL + PITCH_HEIGHT / 2.0])
            .build();
        walls.push(WallEntity::new(cuboid_collider.translation().x, cuboid_collider.translation().y, PITCH_LINE_BREADTH, PITCH_HEIGHT));
        collider_set.insert(cuboid_collider);

        // top pitch line`
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_WIDTH/ 2.0, PITCH_LINE_BREADTH/ 2.0)
            .translation(vector![PITCH_LEFT_WALL + PITCH_WIDTH / 2.0, PITCH_TOP_WALL])
            .build();
        walls.push(WallEntity::new(cuboid_collider.translation().x, cuboid_collider.translation().y, PITCH_WIDTH, PITCH_LINE_BREADTH));
        collider_set.insert(cuboid_collider);

        // bottom pitch line
        let cuboid_collider = ColliderBuilder::cuboid(PITCH_WIDTH/ 2.0, PITCH_LINE_BREADTH/ 2.0)
        .translation(vector![PITCH_LEFT_WALL + PITCH_WIDTH / 2.0, PITCH_BOTTOM_WALL])
        .build();
    walls.push(WallEntity::new(cuboid_collider.translation().x, cuboid_collider.translation().y, PITCH_WIDTH, PITCH_LINE_BREADTH));
    collider_set.insert(cuboid_collider);

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

        let ball_body = &self.rigid_body_set[self.player_body_handle];
    }
    fn parse_player_input(&mut self, input: &PlayerInput) {
        if input.shoot {}
        if input.up {
            let ball_body = &mut self.rigid_body_set[self.player_body_handle];
            let speed = vector![0.0, -5000.0];
            ball_body.apply_impulse(speed, true);
        } else if input.down {
            let ball_body = &mut self.rigid_body_set[self.player_body_handle];
            let speed = vector![0.0, 5000.0];
            ball_body.apply_impulse(speed, true);
        }
        if input.left {
            let ball_body = &mut self.rigid_body_set[self.player_body_handle];
            let speed = vector![-5000.0, 0.0];
            ball_body.apply_impulse(speed, true);
        } else if input.right {
            let ball_body = &mut self.rigid_body_set[self.player_body_handle];
            let speed = vector![5000.0, 0.0];
            ball_body.apply_impulse(speed, true);
        } else {
        }
    }
    pub fn get_ball_entities(&self) -> JsValue {
        let v: Vec<BallEntity> = self.players.iter().map(|player| {
            player.create_entity(&self.rigid_body_set)
        }).collect();
        JsValue::from_serde(&v).unwrap()
    }
    pub fn get_wall_entities(&self) -> JsValue {
        JsValue::from_serde(&self.walls).unwrap()
    }
}

struct Player {
    rigid_body_handle: RigidBodyHandle,
    radius: f32,
    red: bool,
    number: i32,
}

impl Player{
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
