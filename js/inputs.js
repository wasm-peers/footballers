// ==== keys pressed ====

let UP_PRESSED = false;
let DOWN_PRESSED = false;
let LEFT_PRESSED = false;
let RIGHT_PRESSED = false;
let SPACEBAR_PRESSED = false;
let UP_LAST = false;
let LEFT_LAST = false;

// ==== events ====

document.addEventListener('keydown', (event) => {
    let keyName = event.key;

    if (keyName === 'Spacebar' || keyName === ' ') {
        SPACEBAR_PRESSED = true;
    }
    if (keyName === 'w' || keyName === 'ArrowUp') {
        UP_PRESSED = true;
        UP_LAST = true;
    }
    else if (keyName === 's' || keyName === 'ArrowDown') {
        DOWN_PRESSED = true;
        UP_LAST = false;
    }
    else if (keyName === 'a' || keyName === 'ArrowLeft') {
        LEFT_PRESSED = true;
        LEFT_LAST = true;
    }
    else if (keyName === 'd' || keyName === 'ArrowRight') {
        RIGHT_PRESSED = true;
        LEFT_LAST = false;
    }
});

document.addEventListener('keyup', (event) => {
    let keyName = event.key;

    if (keyName === 'Spacebar' || keyName === ' ') {
        SPACEBAR_PRESSED = false;
    }
    if (keyName === 'w' || keyName === 'ArrowUp') {
        UP_PRESSED = false;
        UP_LAST = false;
    }
    else if (keyName === 's' || keyName === 'ArrowDown') {
        DOWN_PRESSED = false;
        UP_LAST = true;
    }
    else if (keyName === 'a' || keyName === 'ArrowLeft') {
        LEFT_PRESSED = false;
        LEFT_LAST = false;
    }
    else if (keyName === 'd' || keyName === 'ArrowRight') {
        RIGHT_PRESSED = false;
        LEFT_LAST = true;
    }
});

// ==== getter function ====

export function getPlayerInput() {
    let input = {
        up: false,
        down: false,
        left: false,
        right: false,
        shoot: SPACEBAR_PRESSED,
    };
    if (UP_PRESSED && UP_LAST) {
        input.up = true;
    }
    else if (DOWN_PRESSED) {
        input.down = true;
    }
    if (LEFT_PRESSED && LEFT_LAST) {
        input.left = true;
    }
    else if (RIGHT_PRESSED) {
        input.right = true;
    }
    return input;
}