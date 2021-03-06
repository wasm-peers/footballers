use crate::components::utils;
use crate::game::{
    ClientGame, FootballersGame, Game, HostGame, GAME_CANVAS_HEIGHT, GAME_CANVAS_WIDTH,
};
use crate::utils::global_window;
use log::error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
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
    game: Option<FootballersGame>,
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
                    let location = global_window().location();
                    let generated_session_id = get_random_session_id();
                    query_params.append("session_id", generated_session_id.as_str());
                    query_params.append("host", "true");
                    let search: String = query_params.to_string().into();
                    if let Err(error) = location.set_search(&search) {
                        error!("Error while setting URL: {error:?}")
                    }
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
                if let Err(e) = copy_link(&self.session_id) {
                    error!("{e:?}");
                }
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
                match self.game.as_mut() {
                    Some(game) => {
                        game.tick();
                        if !game.ended() {
                            if let Err(error) = global_window().request_animation_frame(
                                self.tick_callback.as_ref().unchecked_ref(),
                            ) {
                                error!("Failed requesting next animation frame: {error:?}");
                            }
                        }
                    }
                    None => {
                        error!("No initialized game object yet.");
                    }
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

fn init_game(canvas_node: NodeRef, is_host: bool, session_id: SessionId) -> FootballersGame {
    let context = {
        let canvas = canvas_node
            .cast::<HtmlCanvasElement>()
            .expect("no canvas element on page yet");
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
    let mut game = if is_host {
        FootballersGame::Host(HostGame::new(
            session_id,
            connection_type,
            signaling_server_url,
        ))
    } else {
        FootballersGame::Client(ClientGame::new(
            session_id,
            connection_type,
            signaling_server_url,
        ))
    };
    game.init();
    game
}

fn copy_link(session_id: &SessionId) -> Result<(), JsValue> {
    let window = global_window();
    let clipboard = window
        .navigator()
        .clipboard()
        .ok_or_else(|| JsValue::from("acquiring clipboard failed"))?;
    let location = window.location();
    let origin = location.origin()?;
    let pathname = location.pathname()?;
    let _promise = clipboard.write_text(&format!(
        "{}{}?session_id={}&is_host=false",
        origin,
        pathname,
        session_id.as_str()
    ));
    Ok(())
}
