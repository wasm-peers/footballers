mod client;
mod constants;
mod game;
mod utils;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(module = "/js/rendering.js")]
extern "C" {
    #[wasm_bindgen(js_name = initGame)]
    fn init_game_from_js(session_id: String, is_host: bool);
    #[wasm_bindgen(js_name = draw)]
    fn draw_from_js();
    #[wasm_bindgen(js_name = tick)]
    fn tick_from_js();
    #[wasm_bindgen(js_name = hostSendState)]
    fn host_send_state_from_js();
    #[wasm_bindgen(js_name = checkTimer)]
    fn check_timer_from_js();
}

#[wasm_bindgen(module = "/js/inputs.js")]
extern "C" {
    #[wasm_bindgen(js_name = getPlayerInput)]
    fn get_player_input_from_js() -> JsValue;
}

#[wasm_bindgen]
pub fn main(session_id: String, is_host: bool) {
    console_error_panic_hook::set_once();
    init_game_from_js(session_id, is_host);
}
