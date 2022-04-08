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

#[wasm_bindgen(module = "/js/inputs.js")]
extern "C" {
    #[wasm_bindgen(js_name = getPlayerInput)]
    fn get_local_player_input() -> JsValue;
}

pub trait Game {
    fn init(&mut self);
    fn tick(&mut self);
    fn ended(&self) -> bool;
}
