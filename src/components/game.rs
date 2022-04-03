use crate::components::utils;
use crate::game::constants::{PITCH_HEIGHT, PITCH_WIDTH, PLAYER_DIAMETER};
use crate::game::{ClientGame, HostGame};
use serde::{Deserialize, Serialize};
use wasm_peers::{get_random_session_id, ConnectionType, SessionId};
use yew::{html, Component, Context, Html};

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

pub(crate) struct Game {
    session_id: SessionId,
    is_host: bool,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
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
                    let search: String = query_params.to_string().into();
                    location.set_search(&search).unwrap();
                    (generated_session_id, true)
                }
            };

        let connection_type = ConnectionType::StunAndTurn {
            stun_urls: env!("STUN_SERVER_URLS").to_string(),
            turn_urls: env!("TURN_SERVER_URLS").to_string(),
            username: env!("TURN_SERVER_USERNAME").to_string(),
            credential: env!("TURN_SERVER_CREDENTIAL").to_string(),
        };
        if is_host {
            let mut game = HostGame::new(
                session_id.clone(),
                connection_type,
                concat!(env!("SIGNALING_SERVER_URL"), "/one-to-many"),
            );
            game.start();
        } else {
            let mut game = ClientGame::new(
                session_id.clone(),
                connection_type,
                concat!(env!("SIGNALING_SERVER_URL"), "/one-to-many"),
            );
            game.start();
        };
        Self {
            is_host,
            session_id,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let width = (2.0 * PLAYER_DIAMETER + PITCH_WIDTH + 2.0 * PLAYER_DIAMETER).to_string();
        let height = (2.0 * PLAYER_DIAMETER + PITCH_HEIGHT).to_string();
        html! {
            <main class="px-3">
                <canvas id="canvas" width={ width } height={ height }></canvas>
                <p>{ "Session id:" } { &self.session_id }</p>
                <button id="game_link_button">{ "Copy shareable link" }</button>
            </main>
        }
    }
}
