# Footballers
<img style="display: block; margin-left: auto; margin-right: auto" src="https://i.imgur.com/111ChMK.png" alt="footballers game">

## Description
2D real-time multiplayer game in a browser.
Players divided in two teams play a football match on field with two goal posts.
Goal of the game is for a team to score 3 points before the other team.

This game showcases the usability of [wasm-peers](https://github.com/wasm-peers/wasm-peers) crate for easy and costless peer-2-peer WebRTC communication.

Check the hosted game [here](http://wasm-peers-footballers.s3-website.eu-central-1.amazonaws.com/).

## Functionality
Game supports any number of players, but at least 2 are necessary to start the game.
Players connect by providing session id received by some means from the game host.
This host is responsible for receiving players input, calculating game state and sending updated state to all connected players.

On the field, players can collide with each other and the ball, they can shoot the ball if they are close enough.
If one of the teams scores a goal, by bringing the ball across the goal posts, the score is updated and the game is reset.

## Local development

To run the game locally you must have [Rust](https://www.rust-lang.org/tools/install)
and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.


## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
