// ==== keys pressed ====

let RED_UP_PRESSED = false;
let RED_DOWN_PRESSED = false;
let RED_LEFT_PRESSED = false;
let RED_RIGHT_PRESSED = false;
let RED_SPACEBAR_PRESSED = false;
let RED_UP_LAST = false;
let RED_LEFT_LAST = false;

// ==== events ====

document.addEventListener('keydown', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        RED_SPACEBAR_PRESSED = true;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        RED_UP_PRESSED = true;
        RED_UP_LAST = true;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        RED_DOWN_PRESSED = true;
        RED_UP_LAST = false;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        RED_LEFT_PRESSED = true;
        RED_LEFT_LAST = true;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RED_RIGHT_PRESSED = true;
        RED_LEFT_LAST = false;
    }
});

document.addEventListener('keyup', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        RED_SPACEBAR_PRESSED = false;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        RED_UP_PRESSED = false;
        RED_UP_LAST = false;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        RED_DOWN_PRESSED = false;
        RED_UP_LAST = true;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        RED_LEFT_PRESSED = false;
        RED_LEFT_LAST = false;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RED_RIGHT_PRESSED = false;
        RED_LEFT_LAST = true;
    }
});

// ==== getter function ====

export function getPlayerInput() {
    let input = {
        up: false,
        down: false,
        left: false,
        right: false,
        shoot: RED_SPACEBAR_PRESSED,
    };
    if (RED_UP_PRESSED && RED_UP_LAST) {
        input.up = true;
    }
    else if (RED_DOWN_PRESSED) {
        input.down = true;
    }
    if (RED_LEFT_PRESSED && RED_LEFT_LAST) {
        input.left = true;
    }
    else if (RED_RIGHT_PRESSED) {
        input.right = true;
    }
    return input;
}