use crate::game::utils::{Circle, Edge, Score};
use std::f64::consts;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const PITCH_COLOR: &str = "#619F5E";
const PITCH_LINE_COLOR: &str = "#C7E6BD";
const BALL_COLOR: &str = "#EEEEEE";
const RED_PLAYER_COLOR: &str = "#E56E56";
const BLUE_PLAYER_COLOR: &str = "#5689E5";
const OUTLINE_COLOR: &str = "#000000";
const OUTLINE_WIDTH: f64 = 2.0;
const STADIUM_COLOR: &str = "#718C5A";
const TEXT_COLOR: &str = "#FFFFFF";

pub(crate) fn draw_stadium(
    ctx: &CanvasRenderingContext2d,
    stadium_width: f64,
    stadium_height: f64,
) {
    ctx.set_fill_style(&JsValue::from(STADIUM_COLOR));
    ctx.fill_rect(0.0, 0.0, stadium_width, stadium_height);
}

// TODO: separate this to multiple functions?
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_pitch(
    ctx: &CanvasRenderingContext2d,
    edges: &[Edge],
    pitch_left_line: f64,
    pitch_right_line: f64,
    pitch_top_line: f64,
    pitch_bottom_line: f64,
    pitch_line_width: f64,
    stadium_width: f64,
    stadium_height: f64,
    goal_breadth: f64,
) {
    // green field
    ctx.set_fill_style(&JsValue::from(PITCH_COLOR));
    ctx.fill_rect(
        pitch_left_line,
        pitch_top_line,
        pitch_right_line - pitch_left_line,
        pitch_bottom_line - pitch_top_line,
    );
    ctx.set_line_width(pitch_line_width);
    // pitch white lines
    for edge in edges {
        ctx.set_fill_style(&JsValue::from(if edge.white {
            PITCH_LINE_COLOR
        } else {
            OUTLINE_COLOR
        }));
        ctx.fill_rect(
            edge.x as f64,
            edge.y as f64,
            edge.width as f64,
            edge.height as f64,
        );
    }
    // goals white lines
    ctx.set_fill_style(&JsValue::from(PITCH_LINE_COLOR));
    ctx.set_stroke_style(&JsValue::from(PITCH_LINE_COLOR));
    ctx.begin_path();
    ctx.move_to(pitch_left_line, (stadium_height - goal_breadth) / 2.0);
    ctx.line_to(pitch_left_line, (stadium_height + goal_breadth) / 2.0);
    ctx.stroke();
    ctx.begin_path();
    ctx.move_to(pitch_right_line, (stadium_height - goal_breadth) / 2.0);
    ctx.line_to(pitch_right_line, (stadium_height + goal_breadth) / 2.0);
    ctx.stroke();

    let half_width = stadium_width / 2.0;
    let half_height = stadium_height / 2.0;

    // middle point
    ctx.begin_path();
    ctx.arc(half_width, half_height, 8.0, 0.0, 2.0 * consts::PI)
        .unwrap();
    ctx.close_path();
    ctx.stroke();

    // middle circle
    ctx.begin_path();
    ctx.arc(
        half_width,
        half_height,
        half_height / 3.0,
        0.0,
        2.0 * consts::PI,
    )
    .unwrap();
    ctx.close_path();
    ctx.stroke();

    // middle vertical lines
    ctx.begin_path();
    ctx.move_to(half_width, pitch_top_line);
    ctx.line_to(half_width, pitch_bottom_line);
    ctx.stroke();
}

pub(crate) fn draw_goals(ctx: &CanvasRenderingContext2d, goal_posts: &[Circle]) {
    for goal_post in goal_posts {
        ctx.set_fill_style(&JsValue::from(if goal_post.red {
            RED_PLAYER_COLOR
        } else {
            BLUE_PLAYER_COLOR
        }));
        ctx.begin_path();
        ctx.arc(
            goal_post.x as f64,
            goal_post.y as f64,
            goal_post.radius as f64 - OUTLINE_WIDTH / 2.0,
            0.0,
            2.0 * consts::PI,
        )
        .unwrap();
        ctx.close_path();
        ctx.fill();

        ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
        ctx.set_line_width(OUTLINE_WIDTH);
        ctx.begin_path();
        ctx.arc(
            goal_post.x as f64,
            goal_post.y as f64,
            goal_post.radius as f64 - OUTLINE_WIDTH / 2.0,
            0.0,
            2.0 * consts::PI,
        )
        .unwrap();
        ctx.close_path();
        ctx.stroke();
    }
}

pub(crate) fn draw_score(
    ctx: &CanvasRenderingContext2d,
    score: &Score,
    stadium_width: f64,
    pitch_top_line: f64,
) {
    ctx.set_font("bold 30px arial");
    ctx.set_fill_style(&JsValue::from(PITCH_LINE_COLOR));
    ctx.fill_text(
        &format!("{} - {}", score.red_score, score.blue_score),
        stadium_width / 2.0,
        pitch_top_line / 2.0,
    )
    .unwrap();
}

pub(crate) fn draw_players(ctx: &CanvasRenderingContext2d, players: &[Circle]) {
    for player in players {
        ctx.set_fill_style(&JsValue::from(if player.red {
            RED_PLAYER_COLOR
        } else {
            BLUE_PLAYER_COLOR
        }));
        ctx.begin_path();
        ctx.arc(
            player.x as f64,
            player.y as f64,
            player.radius as f64 - OUTLINE_WIDTH / 2.0,
            0.0,
            2.0 * consts::PI,
        )
        .unwrap();
        ctx.close_path();
        ctx.fill();

        ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
        ctx.set_line_width(OUTLINE_WIDTH);
        ctx.begin_path();
        ctx.arc(
            player.x as f64,
            player.y as f64,
            player.radius as f64 - OUTLINE_WIDTH / 2.0,
            0.0,
            2.0 * consts::PI,
        )
        .unwrap();
        ctx.close_path();
        ctx.stroke();

        // draw number on player
        ctx.set_font("bold 18px arial");
        ctx.set_fill_style(&JsValue::from(TEXT_COLOR));
        ctx.fill_text(
            player.player_number.to_string().as_str(),
            player.x as f64,
            player.y as f64,
        )
        .unwrap();
    }
}

pub(crate) fn draw_ball(ctx: &CanvasRenderingContext2d, ball: &Circle) {
    ctx.set_fill_style(&JsValue::from(BALL_COLOR));
    ctx.begin_path();
    ctx.arc(
        ball.x as f64,
        ball.y as f64,
        ball.radius as f64 - OUTLINE_WIDTH / 2.0,
        0.0,
        2.0 * consts::PI,
    )
    .unwrap();
    ctx.close_path();
    ctx.fill();

    ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
    ctx.set_line_width(OUTLINE_WIDTH);
    ctx.begin_path();
    ctx.arc(
        ball.x as f64,
        ball.y as f64,
        ball.radius as f64 - OUTLINE_WIDTH / 2.0,
        0.0,
        2.0 * consts::PI,
    )
    .unwrap();
    ctx.close_path();
    ctx.stroke();
}

pub(crate) fn draw_red_scored(
    ctx: &CanvasRenderingContext2d,
    stadium_width: f64,
    stadium_height: f64,
) {
    ctx.set_font("bold 42px arial");
    ctx.set_fill_style(&JsValue::from(RED_PLAYER_COLOR));
    ctx.fill_text("Red Scores!", stadium_width / 2.0, stadium_height / 2.0)
        .unwrap();
    ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
    ctx.stroke_text("Red Scores!", stadium_width / 2.0, stadium_height / 2.0)
        .unwrap();
}

pub(crate) fn draw_blue_scored(
    ctx: &CanvasRenderingContext2d,
    stadium_width: f64,
    stadium_height: f64,
) {
    ctx.set_font("bold 42px arial");
    ctx.set_fill_style(&JsValue::from(BLUE_PLAYER_COLOR));
    ctx.fill_text("Blue Scores!", stadium_width / 2.0, stadium_height / 2.0)
        .unwrap();
    ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
    ctx.stroke_text("Blue Scores!", stadium_width / 2.0, stadium_height / 2.0)
        .unwrap();
}

pub(crate) fn draw_game_ended(
    ctx: &CanvasRenderingContext2d,
    score: &Score,
    stadium_width: f64,
    stadium_height: f64,
) {
    let half_text_height = 21.0;
    ctx.set_font("bold 42px arial");
    ctx.set_stroke_style(&JsValue::from(OUTLINE_COLOR));
    if score.red_score > score.blue_score {
        ctx.set_fill_style(&JsValue::from(RED_PLAYER_COLOR));
        ctx.fill_text(
            "Red Won!",
            stadium_width / 2.0,
            stadium_height / 2.0 - half_text_height,
        )
        .unwrap();
        ctx.stroke_text(
            "Red Won!",
            stadium_width / 2.0,
            stadium_height / 2.0 - half_text_height,
        )
        .unwrap();
    } else {
        ctx.set_fill_style(&JsValue::from(BLUE_PLAYER_COLOR));
        ctx.fill_text(
            "Blue Won!",
            stadium_width / 2.0,
            stadium_height / 2.0 - half_text_height,
        )
        .unwrap();
        ctx.stroke_text(
            "Blue Won!",
            stadium_width / 2.0,
            stadium_height / 2.0 - half_text_height,
        )
        .unwrap();
    }
    ctx.set_fill_style(&JsValue::from(TEXT_COLOR));
    ctx.fill_text(
        &format!("{} - {}", score.red_score, score.blue_score),
        stadium_width / 2.0,
        stadium_height / 2.0 + half_text_height,
    )
    .unwrap();
    ctx.stroke_text(
        &format!("{} - {}", score.red_score, score.blue_score),
        stadium_width / 2.0,
        stadium_height / 2.0 + half_text_height,
    )
    .unwrap();
}
