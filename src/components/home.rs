use crate::components::game::GameQuery;
use crate::components::{utils, Route};
use wasm_peers::get_random_session_id;
use yew::prelude::*;
use yew_router::prelude::*;

pub(crate) enum HomeMsg {
    UpdateInput,
}

pub(crate) struct Home {
    input: String,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateInput => {
                self.input = utils::get_input("join-input").value();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let start_as_host = {
            let history = history.clone();
            Callback::once(move |_| {
                history
                    .push_with_query(
                        Route::Game,
                        GameQuery::new(get_random_session_id().into_inner(), true),
                    )
                    .unwrap();
            })
        };
        let update_input = ctx.link().callback(|_| HomeMsg::UpdateInput);
        let join_existing = {
            let session_id = self.input.clone();
            Callback::once(move |_| {
                if !session_id.is_empty() {
                    history
                        .push_with_query(Route::Game, GameQuery::new(session_id, false))
                        .unwrap();
                }
            })
        };
        html! {
                <div class="cover-container d-flex w-100 h-100 p-3 mx-auto flex-column">
                    <header class="mb-auto">
                        <div>
                            <h3 class="float-md-start mb-0">{ "Footballers" }</h3>
                        </div>
                    </header>

                    <main class="px-3">
                        <h1>{ "Footballers" }</h1>

                        <p class="lead">{ "2D real-time multiplayer game in a browser." }</p>
                        <p class="lead">{ "Players divided in two teams play a football match on field with two goal posts." }</p>
                        <p class="lead">{ "Goal of the game is for a team to score 3 points before the other team." }</p>
                        <p class="lead">{ "Use WASD to move, SPACE to shoot the ball." }</p>
                        <hr />
                        <p class="lead">
                            <button onclick={ start_as_host } class="btn btn-lg btn-secondary fw-bold border-white bg-white">{ "Start game as host" }</button>
                        </p>
                        <p class="lead">{ "or join existing game" }</p>
                        <p class="lead">
                        <input id="join-input"
                            placeholder={ "Session id from a friend" }
                            oninput={ update_input }
                        />
                        </p>
                        <p class="lead">
                            <button onclick={ join_existing } class="btn btn-lg btn-secondary fw-bold border-white bg-white">{ "Join existing" }</button>
                        </p>
                    </main>
                </div>
        }
    }
}
