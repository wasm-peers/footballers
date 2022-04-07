mod client;
mod constants;
mod host;
mod rendering;
mod utils;

use crate::game::constants::{PITCH_HEIGHT, PITCH_WIDTH, PLAYER_DIAMETER};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub use crate::game::client::ClientGame;
pub use crate::game::host::HostGame;

pub const GAME_CANVAS_WIDTH: f32 = 2.0 * PLAYER_DIAMETER + PITCH_WIDTH + 2.0 * PLAYER_DIAMETER;
pub const GAME_CANVAS_HEIGHT: f32 = 2.0 * PLAYER_DIAMETER + PITCH_HEIGHT;

#[wasm_bindgen(module = "/js/rendering.js")]
extern "C" {
    #[wasm_bindgen(js_name = drawPitch)]
    fn draw_pitch(
        edges: JsValue,
        pitch_left_line: f32,
        pitch_right_line: f32,
        pitch_top_line: f32,
        pitch_bottom_line: f32,
        pitch_line_width: f32,
        stadium_width: f32,
        stadium_height: f32,
        goal_breadth: f32,
    );
    #[wasm_bindgen(js_name = drawGoals)]
    fn draw_goals(goal_posts: JsValue);
    #[wasm_bindgen(js_name = drawScore)]
    fn draw_score(score: JsValue, stadium_width: f32, pitch_top_line: f32);
    #[wasm_bindgen(js_name = drawPlayers)]
    fn draw_players(players: JsValue);
    #[wasm_bindgen(js_name = drawBall)]
    fn draw_ball(ball: JsValue);
    #[wasm_bindgen(js_name = drawRedScored)]
    fn draw_red_scored(stadium_width: f32, stadium_height: f32);
    #[wasm_bindgen(js_name = drawBlueScored)]
    fn draw_blue_scored(stadium_width: f32, stadium_height: f32);
    #[wasm_bindgen(js_name = drawGameEnded)]
    fn draw_game_ended(score: JsValue, stadium_width: f32, stadium_height: f32);
}

#[wasm_bindgen(module = "/js/inputs.js")]
extern "C" {
    #[wasm_bindgen(js_name = getPlayerInput)]
    fn get_local_player_input() -> JsValue;
}

pub trait Game {
    fn init(&mut self);
    fn tick(&mut self);
}
