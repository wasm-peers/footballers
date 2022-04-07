use crate::game::constants::{
    BALL_RADIUS, GOAL_BREADTH, PITCH_BOTTOM_LINE, PITCH_LEFT_LINE, PITCH_LINE_WIDTH,
    PITCH_RIGHT_LINE, PITCH_TOP_LINE, RESET_TIME, STADIUM_HEIGHT, STADIUM_WIDTH,
};
use crate::game::utils::{Circle, Edge, Message, PlayerInput, Score};
use crate::game::{rendering, Game};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_peers::one_to_many::MiniClient;
use wasm_peers::{ConnectionType, SessionId};
use web_sys::CanvasRenderingContext2d;

pub struct ClientGame {
    inner: Rc<RefCell<ClientGameInner>>,
}

impl ClientGame {
    pub fn new(
        session_id: SessionId,
        connection_type: ConnectionType,
        signaling_server_url: &str,
    ) -> Self {
        ClientGame {
            inner: Rc::new(RefCell::new(ClientGameInner::new(
                session_id,
                connection_type,
                signaling_server_url,
            ))),
        }
    }
}

impl Game for ClientGame {
    fn init(&mut self) {
        let on_open_callback = || {};

        let inner = self.inner.clone();
        let on_message_callback = move |message: String| {
            let message = serde_json::from_str::<Message>(&message).unwrap();

            match message {
                Message::GameInit {
                    edges,
                    goal_posts,
                    players,
                    ball,
                } => {
                    inner.borrow_mut().edges = edges;
                    inner.borrow_mut().goal_posts = goal_posts;
                    inner.borrow_mut().players = players;
                    inner.borrow_mut().ball = ball;
                }
                Message::GameState { players, ball } => {
                    inner.borrow_mut().players = players;
                    inner.borrow_mut().ball = ball;
                }
                Message::GoalScored { score, red_scored } => {
                    inner.borrow_mut().score = score;
                    inner.borrow_mut().red_scored = red_scored;
                    inner.borrow_mut().blue_scored = !red_scored;
                    inner.borrow_mut().timer = RESET_TIME;
                }
                Message::GameEnded => {
                    inner.borrow_mut().game_ended = true;
                }
            }
        };

        self.inner
            .borrow_mut()
            .mini_client
            .start(on_open_callback, on_message_callback)
            .expect("network manager failed to start");
    }

    fn tick(&mut self) {
        self.inner.borrow_mut().tick();
    }
}

struct ClientGameInner {
    mini_client: MiniClient,
    edges: Vec<Edge>,
    goal_posts: Vec<Circle>,
    players: Vec<Circle>,
    ball: Circle,
    score: Score,
    red_scored: bool,
    blue_scored: bool,
    game_ended: bool,
    timer: u32,
}

impl ClientGameInner {
    pub(self) fn new(
        session_id: SessionId,
        connection_type: ConnectionType,
        signaling_server_url: &str,
    ) -> Self {
        let mini_client = MiniClient::new(signaling_server_url, session_id, connection_type)
            .expect("failed to create network manager");
        ClientGameInner {
            mini_client,
            edges: Vec::new(),
            goal_posts: Vec::new(),
            players: Vec::new(),
            ball: Circle::new(0.0, 0.0, BALL_RADIUS, false, -1),
            score: Score::new(0, 0),
            red_scored: false,
            blue_scored: false,
            game_ended: false,
            timer: 0,
        }
    }

    fn tick(&mut self) {
        if self.timer == 0 {
            self.red_scored = false;
            self.blue_scored = false;
        } else {
            self.timer -= 1;
        }

        // on each frame, send input to host
        let message = serde_json::to_string::<PlayerInput>(
            &crate::game::get_local_player_input().into_serde().unwrap(),
        )
        .unwrap();

        // allow some messages to fail
        let _ = self.mini_client.send_message_to_host(&message);

        self.draw();
    }

    fn draw(&self) {
        // rendering::draw_stadium(&self.context, STADIUM_WIDTH.into(), STADIUM_HEIGHT.into());
        crate::game::draw_pitch(
            JsValue::from_serde(&self.edges).unwrap(),
            PITCH_LEFT_LINE,
            PITCH_RIGHT_LINE,
            PITCH_TOP_LINE,
            PITCH_BOTTOM_LINE,
            PITCH_LINE_WIDTH,
            STADIUM_WIDTH,
            STADIUM_HEIGHT,
            GOAL_BREADTH,
        );
        crate::game::draw_goals(JsValue::from_serde(&self.goal_posts).unwrap());
        crate::game::draw_score(
            JsValue::from_serde(&self.score).unwrap(),
            STADIUM_WIDTH,
            PITCH_TOP_LINE,
        );
        crate::game::draw_players(JsValue::from_serde(&self.players).unwrap());
        crate::game::draw_ball(JsValue::from_serde(&self.ball).unwrap());
        if self.red_scored {
            crate::game::draw_red_scored(STADIUM_WIDTH, STADIUM_HEIGHT);
        }
        if self.blue_scored {
            crate::game::draw_blue_scored(STADIUM_WIDTH, STADIUM_HEIGHT);
        }
        if self.game_ended {
            crate::game::draw_game_ended(
                JsValue::from_serde(&self.score).unwrap(),
                STADIUM_WIDTH,
                STADIUM_HEIGHT,
            );
        }
    }
}