# Footballers
<img style="display: block; margin-left: auto; margin-right: auto" src="https://i.imgur.com/111ChMK.png" alt="footballers game">

## Description
2D real-time multiplayer game in a browser.
Players divided in two teams play a football match on field with two goal posts.
Goal of the game is for a team to score 3 points before the other team.

This game showcases the usability of [wasm-peers](https://github.com/wasm-peers/wasm-peers#readme) crate for easy and costless peer-2-peer WebRTC communication.

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
Also, [npm](https://docs.npmjs.com/cli/v8/configuring-npm/install) is required as well.

Signaling server from wasm-peer project should be running on `0.0.0.0:9001`.
See [here](https://github.com/wasm-peers/wasm-peers/tree/main/signaling-server) for instructions.

First, some env variables are required:
```bash
# setting required variables
export SIGNALING_SERVER_URL="ws://0.0.0.0:9001"
export STUN_SERVER_URLS="stun:stun.l.google.com:19302"

# these are dummy values,
# if you want to deploy the game yourself, you would need to provide TURN server url and credentials
export TURN_SERVER_URLS="turn:dummy.com"
export TURN_SERVER_USERNAME="dummy"
export TURN_SERVER_CREDENTIAL="dummy"
```

Then you can build the project:
```bash
wasm-pack build
cd webpage
npm run build # npm run start, if you want hot-reloading and serving
```

This will create a `webpage/dist` folder with `index.html` and all the other required files. 
You can serve them any way you like.

## Authors

Arkadiusz Górecki  
[LinkedIn](https://www.linkedin.com/in/arkadiusz-gorecki/)

Tomasz Karwowski  
[LinkedIn](https://www.linkedin.com/in/tomek-karwowski/)

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
