use crate::components::utils;
use crate::game::{ClientGame, Game, HostGame, GAME_CANVAS_HEIGHT, GAME_CANVAS_WIDTH};
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_peers::{get_random_session_id, ConnectionType, SessionId};
use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef};

#[derive(Serialize, Deserialize)]
pub struct GameQuery {
    pub session_id: String,
    pub is_host: bool,
}

impl GameQuery {
    pub(crate) fn new(session_id: String, is_host: bool) -> Self {
        GameQuery {
            session_id,
            is_host,
        }
    }
}

pub enum GameMsg {
    CopyLink,
    Init,
    Tick,
}

pub(crate) struct GameComponent {
    session_id: SessionId,
    is_host: bool,
    canvas: NodeRef,
    game: Option<Box<dyn Game>>,
    tick_callback: Closure<dyn FnMut()>,
}

impl Component for GameComponent {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let query_params = utils::get_query_params();
        let (session_id, is_host) =
            match (query_params.get("session_id"), query_params.get("is_host")) {
                (Some(session_string), Some(is_host)) => {
                    (SessionId::new(session_string), is_host == "true")
                }
                _ => {
                    let location = web_sys::window().unwrap().location();
                    let generated_session_id = get_random_session_id();
                    query_params.append("session_id", generated_session_id.as_str());
                    query_params.append("host", "true");
                    let search: String = query_params.to_string().into();
                    location.set_search(&search).unwrap();
                    (generated_session_id, true)
                }
            };
        let canvas = NodeRef::default();
        let tick_callback = {
            let link = ctx.link().clone();
            Closure::wrap(Box::new(move || link.send_message(GameMsg::Tick)) as Box<dyn FnMut()>)
        };
        ctx.link().send_message(GameMsg::Init);
        Self {
            is_host,
            session_id,
            canvas,
            game: None,
            tick_callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::CopyLink => {
                let window = web_sys::window().unwrap();
                let clipboard = window.navigator().clipboard().unwrap();
                let location = window.location();
                let origin = location.origin().unwrap();
                let pathname = location.pathname().unwrap();
                let _promise = clipboard.write_text(&format!(
                    "{}{}?session_id={}&is_host=false",
                    origin, pathname, self.session_id
                ));
                false
            }
            GameMsg::Init => {
                self.game = Some(init_game(
                    self.canvas.clone(),
                    self.is_host,
                    self.session_id.clone(),
                ));
                ctx.link().send_message(GameMsg::Tick);
                false
            }
            GameMsg::Tick => {
                if !self.game.as_ref().unwrap().ended() {
                    self.game.as_mut().unwrap().tick();
                    web_sys::window()
                        .unwrap()
                        .request_animation_frame(self.tick_callback.as_ref().unchecked_ref())
                        .unwrap();
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = GAME_CANVAS_WIDTH.to_string();
        let height = GAME_CANVAS_HEIGHT.to_string();
        let onclick = ctx.link().callback(|_| GameMsg::CopyLink);
        html! {
            <div class="px-3">
                <canvas id="canvas" { width } { height } ref={ self.canvas.clone() }></canvas>
                <p class="lead">{ "Use WASD to move, SPACE to shoot the ball." }</p>
                <p class="lead">{ "Session id:" } { &self.session_id }</p>
                <button id="game_link_button" { onclick }>{ "Copy shareable link" }</button>
            </div>
        }
    }
}

fn init_game(canvas: NodeRef, is_host: bool, session_id: SessionId) -> Box<dyn Game> {
    let context = {
        let canvas = canvas.cast::<HtmlCanvasElement>().unwrap();

        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    };
    context.set_text_align("center");
    context.set_text_baseline("middle");

    let connection_type = ConnectionType::StunAndTurn {
        stun_urls: env!("STUN_SERVER_URLS").to_string(),
        turn_urls: env!("TURN_SERVER_URLS").to_string(),
        username: env!("TURN_SERVER_USERNAME").to_string(),
        credential: env!("TURN_SERVER_CREDENTIAL").to_string(),
    };
    let signaling_server_url = concat!(env!("SIGNALING_SERVER_URL"), "/one-to-many");
    let mut game: Box<dyn Game> = if is_host {
        Box::new(HostGame::new(
            session_id,
            connection_type,
            signaling_server_url,
        ))
    } else {
        Box::new(ClientGame::new(
            session_id,
            connection_type,
            signaling_server_url,
        ))
    };
    game.init();
    game
}
