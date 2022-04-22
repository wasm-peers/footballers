use crate::game::constants::{
    BALL_RADIUS, GOAL_BREADTH, PITCH_BOTTOM_LINE, PITCH_LEFT_LINE, PITCH_LINE_WIDTH,
    PITCH_RIGHT_LINE, PITCH_TOP_LINE, RESET_TIME, STADIUM_HEIGHT, STADIUM_WIDTH,
};
use crate::game::input::PlayerInput;
use crate::game::utils::{Circle, Edge, Message, Score};
use crate::game::{input, rendering, Game};
use crate::utils::global_window;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
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

    fn ended(&self) -> bool {
        self.inner.borrow().game_ended
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
    context: CanvasRenderingContext2d,
    player_input: Rc<RefCell<PlayerInput>>,
}

impl ClientGameInner {
    pub(self) fn new(
        session_id: SessionId,
        connection_type: ConnectionType,
        signaling_server_url: &str,
    ) -> Self {
        let mini_client = MiniClient::new(signaling_server_url, session_id, connection_type)
            .expect("failed to create network manager");

        let document = global_window().document().unwrap();
        let context = {
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap()
        };
        context.set_text_align("center");
        context.set_text_baseline("middle");

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
            context,
            player_input: input::local_player_input(),
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
        let message = serde_json::to_string::<PlayerInput>(&self.player_input.borrow()).unwrap();

        // allow some messages to fail
        let _ = self.mini_client.send_message_to_host(&message);

        self.draw();
    }

    fn draw(&self) {
        rendering::draw_stadium(&self.context, STADIUM_WIDTH as f64, STADIUM_HEIGHT as f64);
        rendering::draw_pitch(
            &self.context,
            &self.edges,
            PITCH_LEFT_LINE as f64,
            PITCH_RIGHT_LINE as f64,
            PITCH_TOP_LINE as f64,
            PITCH_BOTTOM_LINE as f64,
            PITCH_LINE_WIDTH as f64,
            STADIUM_WIDTH as f64,
            STADIUM_HEIGHT as f64,
            GOAL_BREADTH as f64,
        );
        rendering::draw_goals(&self.context, &self.goal_posts);
        rendering::draw_score(
            &self.context,
            &self.score,
            STADIUM_WIDTH as f64,
            PITCH_TOP_LINE as f64,
        );
        rendering::draw_players(&self.context, &self.players);
        rendering::draw_ball(&self.context, &self.ball);
        if self.red_scored {
            rendering::draw_red_scored(&self.context, STADIUM_WIDTH as f64, STADIUM_HEIGHT as f64);
        }
        if self.blue_scored {
            rendering::draw_blue_scored(&self.context, STADIUM_WIDTH as f64, STADIUM_HEIGHT as f64);
        }
        if self.game_ended {
            rendering::draw_game_ended(
                &self.context,
                &self.score,
                STADIUM_WIDTH as f64,
                STADIUM_HEIGHT as f64,
            );
        }
    }
}
