import * as wasm from "../pkg/football_game.js";

const idParagraph = document.getElementById('session_id');
const buttons = document.getElementById('buttons');
const startButton = document.getElementById('start_button');
const joinButton = document.getElementById('join_button');
const idInput = document.getElementById('id_input');

const params = new URLSearchParams(window.location.search);
if (params.has("session_id")) {
    buttons.style.visibility = "hidden";
    wasm.main(params.get("session_id"), params.get("is_host") === "true");
} else {
    startButton.addEventListener('click', event => {
        buttons.style.visibility = "hidden";
        let sessionId = wasm.get_random_session_id();
        idParagraph.innerText = sessionId;
        wasm.main(sessionId, true);
    });

    joinButton.addEventListener('click', event => {
        buttons.style.visibility = "hidden";
        let sessionId = idInput.value;
        wasm.main(sessionId, false);
    });
}