use crate::constants::{
    BALL_RADIUS, GOAL_BREADTH, PITCH_BOTTOM_LINE, PITCH_LEFT_LINE, PITCH_LINE_WIDTH,
    PITCH_RIGHT_LINE, PITCH_TOP_LINE, RESET_TIME, STADIUM_HEIGHT, STADIUM_WIDTH,
};
use crate::utils::{Edge, Message, PlayerInput, Score};
use crate::Circle;
use rusty_games_library::one_to_many::MiniClient;
use rusty_games_library::{ConnectionType, SessionId};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

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

    pub(crate) fn start(&mut self) {
        let inner = self.inner.clone();
        let on_open_callback = move |_| {};

        let inner = self.inner.clone();
        let on_message_callback = move |_, message: String| {
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
                    let inner = inner.clone();
                    let g = Closure::wrap(Box::new(move || {
                        if inner.borrow().timer == 0 {
                            inner.borrow_mut().red_scored = false;
                            inner.borrow_mut().blue_scored = false;
                        } else {
                            inner.borrow_mut().timer -= 1;
                        }
                        // crate::check_timer_from_js();

                        // on each frame, send input to host
                        let message = serde_json::to_string::<PlayerInput>(
                            &crate::get_local_player_input().into_serde().unwrap(),
                        )
                        .unwrap();
                        // allow some messages to fail
                        let _ = inner.borrow().mini_client.send_message_to_host(&message);

                        inner.borrow().draw();
                    }) as Box<dyn FnMut()>);
                    crate::utils::set_interval_with_callback(&g);
                    g.forget();
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
    pub fn new(
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

    fn draw(&self) {
        crate::draw_stadium(STADIUM_WIDTH, STADIUM_HEIGHT);
        crate::draw_pitch(
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
        crate::draw_goals(JsValue::from_serde(&self.goal_posts).unwrap());
        crate::draw_score(
            JsValue::from_serde(&self.score).unwrap(),
            STADIUM_WIDTH,
            PITCH_TOP_LINE,
        );
        crate::draw_players(JsValue::from_serde(&self.players).unwrap());
        crate::draw_ball(JsValue::from_serde(&self.ball).unwrap());
        if self.red_scored {
            crate::draw_red_scored(STADIUM_WIDTH, STADIUM_HEIGHT);
        }
        if self.blue_scored {
            crate::draw_blue_scored(STADIUM_WIDTH, STADIUM_HEIGHT);
        }
        if self.game_ended {
            crate::draw_game_ended(
                JsValue::from_serde(&self.score).unwrap(),
                STADIUM_WIDTH,
                STADIUM_HEIGHT,
            );
        }
    }
}
