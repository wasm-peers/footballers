import { Game } from "football-game";

// function for debugging and printing things from rust
export function log(text) {
    return console.log(text);
}
export function alert(text) {
    return alert(text);
}

// constants
const BALL_COLOR = '#EEEEEE';
const LINE_COLOR = '#EEEEEE';
const RED_COLOR = '#EE4444';
const BLUE_COLOR = '#4444EE';
const OUTLINE_WIDTH = 2;

let game = Game.new();

const WIDTH = 1000;
const HEIGHT = 1000;

// initialization
var ctx = document.getElementById('canvas');
ctx.setAttribute('width', WIDTH)
ctx.setAttribute('height', HEIGHT)
var ctx = ctx.getContext('2d');
ctx.fillStyle = BALL_COLOR;
ctx.strokeStyle = BALL_COLOR;
ctx.lineWidth = OUTLINE_WIDTH;

function drawStadium() {
    let walls = game.get_wall_entities();

    walls.forEach(wall => {
        ctx.fillStyle = LINE_COLOR;
        ctx.fillRect(wall.x, wall.y, wall.width, wall.height);
    })
}

function drawBall() {
    let balls = game.get_ball_entities();
    balls.forEach(ball => {
        if (ball.red) {
            ctx.strokeStyle = RED_COLOR;
        } else {
            ctx.strokeStyle = BLUE_COLOR;
        }
        ctx.beginPath();
        ctx.arc(ball.x, ball.y, ball.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
        ctx.closePath();
        ctx.stroke();
    })
}

function draw() {
    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    drawStadium();
    drawBall();
}

// events
let UP_PRESSED = false;
let DOWN_PRESSED = false;
let LEFT_PRESSED = false;
let RIGHT_PRESSED = false;
let SPACEBAR_PRESSED = false;
let UP_LAST = false;
let LEFT_LAST = false;

function getInput() {
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

// game loop
async function loop() {
    game.tick(getInput());
    draw();

    requestAnimationFrame(loop);
}

document.addEventListener('keydown', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        SPACEBAR_PRESSED = true;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        UP_PRESSED = true;
        UP_LAST = true;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        DOWN_PRESSED = true;
        UP_LAST = false;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        LEFT_PRESSED = true;
        LEFT_LAST = true;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RIGHT_PRESSED = true;
        LEFT_LAST = false;
    }
});

document.addEventListener('keyup', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        SPACEBAR_PRESSED = false;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        UP_PRESSED = false;
        UP_LAST = false;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        DOWN_PRESSED = false;
        UP_LAST = true;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        LEFT_PRESSED = false;
        LEFT_LAST = false;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RIGHT_PRESSED = false;
        LEFT_LAST = true;
    }
});

// start
requestAnimationFrame(loop);
