use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const PITCH_COLOR: &str = "#619F5E";
const PITCH_LINE_COLOR: &str = "#C7E6BD";
const BALL_COLOR: &str = "#EEEEEE";
const RED_PLAYER_COLOR: &str = "#E56E56";
const BLUE_PLAYER_COLOR: &str = "#5689E5";
const OUTLINE_COLOR: &str = "#000000";
const OUTLINE_WIDTH: u8 = 2;
const STADIUM_COLOR: &str = "#718C5A";
const TEXT_COLOR: &str = "#FFFFFF";

pub(crate) fn draw_all(ctx: &CanvasRenderingContext2d) {}

pub(crate) fn draw_stadium(
    ctx: &CanvasRenderingContext2d,
    stadium_width: f64,
    stadium_height: f64,
) {
    ctx.set_fill_style(&JsValue::from(STADIUM_COLOR));
    ctx.fill_rect(0.0, 0.0, stadium_width, stadium_height);
}
