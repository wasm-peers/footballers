mod client;
mod constants;
mod host;
mod input;
mod rendering;
mod utils;

use crate::game::constants::{PITCH_HEIGHT, PITCH_WIDTH, PLAYER_DIAMETER};

pub use crate::game::client::ClientGame;
pub use crate::game::host::HostGame;

pub const GAME_CANVAS_WIDTH: f32 = 2.0 * PLAYER_DIAMETER + PITCH_WIDTH + 2.0 * PLAYER_DIAMETER;
pub const GAME_CANVAS_HEIGHT: f32 = 2.0 * PLAYER_DIAMETER + PITCH_HEIGHT;

pub trait Game {
    fn init(&mut self);
    fn tick(&mut self);
    fn ended(&self) -> bool;
}

pub enum FootballersGame {
    Host(HostGame),
    Client(ClientGame),
}

impl Game for FootballersGame {
    fn init(&mut self) {
        match self {
            FootballersGame::Host(game) => game.init(),
            FootballersGame::Client(game) => game.init(),
        }
    }

    fn tick(&mut self) {
        match self {
            FootballersGame::Host(game) => game.tick(),
            FootballersGame::Client(game) => game.tick(),
        }
    }

    fn ended(&self) -> bool {
        match self {
            FootballersGame::Host(game) => game.ended(),
            FootballersGame::Client(game) => game.ended(),
        }
    }
}
