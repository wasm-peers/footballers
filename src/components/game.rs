use crate::components::utils;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_peers::many_to_many::NetworkManager;
use wasm_peers::{get_random_session_id, ConnectionType, SessionId};
use yew::{html, Component, Context, Html};

pub(crate) enum DocumentMsg {
    UpdateValue,
}

#[derive(Serialize, Deserialize)]
pub struct GameQuery {
    pub session_id: String,
}

impl GameQuery {
    pub(crate) fn new(session_id: String) -> Self {
        GameQuery { session_id }
    }
}

pub(crate) struct Game {
    session_id: SessionId,
    network_manager: NetworkManager,
    is_ready: Rc<RefCell<bool>>,
}

const TEXTAREA_ID: &str = "document-textarea";

impl Component for Game {
    type Message = DocumentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let query_params = utils::get_query_params();
        let session_id = match query_params.get("session_id") {
            Some(session_string) => SessionId::new(session_string),
            None => {
                let location = web_sys::window().unwrap().location();
                let generated_session_id = get_random_session_id();
                query_params.append("session_id", generated_session_id.as_str());
                let search: String = query_params.to_string().into();
                location.set_search(&search).unwrap();
                generated_session_id
            }
        };

        let is_ready = Rc::new(RefCell::new(false));
        let connection_type = ConnectionType::StunAndTurn {
            stun_urls: env!("STUN_SERVER_URLS").to_string(),
            turn_urls: env!("TURN_SERVER_URLS").to_string(),
            username: env!("TURN_SERVER_USERNAME").to_string(),
            credential: env!("TURN_SERVER_CREDENTIAL").to_string(),
        };
        let mut network_manager = NetworkManager::new(
            concat!(env!("SIGNALING_SERVER_URL"), "/many-to-many"),
            session_id.clone(),
            connection_type,
        )
        .unwrap();
        let on_open_callback = {
            let mini_server = network_manager.clone();
            let is_ready = is_ready.clone();
            move |user_id| {
                if !*is_ready.borrow() {
                    utils::get_text_area(TEXTAREA_ID).set_disabled(false);
                    utils::get_text_area(TEXTAREA_ID).set_placeholder("This is a live document shared with other users.\nWhat you write will be visible to everyone.");
                    *is_ready.borrow_mut() = true;
                }
                let value = utils::get_text_area(TEXTAREA_ID).value();
                if !value.is_empty() {
                    mini_server
                        .send_message(user_id, &value)
                        .expect("failed to send current input to new connection");
                }
            }
        };
        let on_message_callback = {
            move |_, message: String| {
                utils::get_text_area(TEXTAREA_ID).set_value(&message);
            }
        };
        network_manager
            .start(on_open_callback, on_message_callback)
            .expect("mini server failed to start");
        Self {
            is_ready,
            network_manager,
            session_id,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateValue => {
                let textarea_value = utils::get_text_area(TEXTAREA_ID).value();
                self.network_manager.send_message_to_all(&textarea_value);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|_| Self::Message::UpdateValue);
        let disabled = !*self.is_ready.borrow();
        let placeholder = "This is a live document shared with other users.\nYou will be allowed to write once other join, or your connection is established.";
        html! {
            <main class="px-3">
                <p class="lead"> { "Share session id: " } <span class="line">{ &self.session_id }</span> </p>
                <p class="lead"> { "or just copy the page url." } </p>
                <textarea id={ TEXTAREA_ID } class="document" cols="100" rows="30" { disabled } { placeholder } { oninput }/>
            </main>
        }
    }
}
