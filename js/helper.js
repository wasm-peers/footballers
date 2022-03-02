import * as wasm from "../pkg/footballers";

const sessionIdParagraph = document.getElementById('session_id_paragraph');
const idParagraph = document.getElementById('session_id');
const gameLinkButton = document.getElementById('game_link_button');
const buttons = document.getElementById('buttons');
const startButton = document.getElementById('start_button');
const joinButton = document.getElementById('join_button');
const idInput = document.getElementById('id_input');

async function copyToClipboard(sessionId) {
    const link = `${window.location.href}?session_id=${sessionId}&is_host=false`;
    await navigator.clipboard.writeText(link);
}

const params = new URLSearchParams(window.location.search);
if (params.has("session_id")) {
    buttons.style.visibility = "hidden";
    wasm.main(params.get("session_id"), params.get("is_host") === "true");
} else {
    startButton.addEventListener('click', event => {
        let sessionId = wasm.get_random_session_id();
        buttons.style.visibility = "hidden";
        sessionIdParagraph.hidden = false;
        idParagraph.innerText = sessionId;
        gameLinkButton.hidden = false;
        gameLinkButton.onclick = () => copyToClipboard(sessionId);
        wasm.main(sessionId, true);
    });

    joinButton.addEventListener('click', event => {
        let sessionId = idInput.value;
        buttons.style.visibility = "hidden";
        sessionIdParagraph.hidden = false;
        idParagraph.innerText = sessionId;
        gameLinkButton.hidden = false;
        gameLinkButton.onclick = () => copyToClipboard(sessionId);
        wasm.main(sessionId, false);
    });
}