
import * as wasm from "../pkg/football_game.js";

const idParagraph = document.getElementById('session_id');
const buttons = document.getElementById('buttons');
const startButton = document.getElementById('start_button');
const joinButton = document.getElementById('join_button');
const idInput = document.getElementById('id_input');

startButton.addEventListener('click', event => {
    buttons.style.visibility = "hidden";
    let sessionId = wasm.get_random_session_id();
    idParagraph.innerText = sessionId;
    wasm.main(sessionId, true);
});

joinButton.addEventListener('click', event => {
    buttons.style.visibility = "hidden";
    let id = idInput.value;
    wasm.main(id, false);
});