import { Game } from "football-game";

// constants
const PITCH_COLOR = '#006600';
const PITCH_LINE_COLOR = '#AAAAAA';
const BALL_COLOR = '#EEEEEE';
const PLAYER_COLOR = '#CC0000';

let game = Game.new();
const WIDTH = game.width();
const HEIGHT = game.height();
const PITCH_LINE_WIDTH = game.pitch_line_width();

// initialization
var ctx = document.getElementById('canvas');
ctx.setAttribute('width', WIDTH)
ctx.setAttribute('height', HEIGHT)
var ctx = ctx.getContext('2d');

function drawPitch() {
    // green field
    ctx.fillStyle = PITCH_COLOR;
    ctx.fillRect(0, 0, WIDTH, HEIGHT);

    // set styles for lines
    ctx.lineWidth = PITCH_LINE_WIDTH;
    ctx.fillStyle = PITCH_LINE_COLOR
    ctx.strokeStyle = PITCH_LINE_COLOR;
    const halfW = WIDTH / 2;
    const halfH = HEIGHT / 2;

    // boundaries
    ctx.strokeRect(0, 0, WIDTH, HEIGHT);

    ctx.moveTo(halfW, halfH);

    // middle point
    ctx.beginPath();
    ctx.arc(halfW, halfH, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.fill();

    // middle circle
    ctx.beginPath();
    ctx.arc(halfW, halfH, halfH / 2, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();

    // middle vertical lines
    ctx.beginPath()
    ctx.moveTo(halfW, 0);
    ctx.lineTo(halfW, HEIGHT);
    ctx.stroke()

    ctx.lineWidth = 1;
}

function drawBall() {
    ctx.fillStyle = BALL_COLOR;
    ctx.beginPath();
    ctx.arc(game.ball_x(), game.ball_y(), game.ball_radius(), 0, 2 * Math.PI, true);
    ctx.closePath();
    ctx.fill();
}

function drawPlayer() {
    ctx.fillStyle = PLAYER_COLOR;
    ctx.beginPath();
    ctx.arc(game.player_x(), game.player_y(), game.player_radius(), 0, 2 * Math.PI, true);
    ctx.closePath();
    ctx.fill();
}

function draw() {
    drawPitch();
    drawBall();
    drawPlayer();
}

// events
let UP_PRESSED = false;
let DOWN_PRESSED = false;
let LEFT_PRESSED = false;
let RIGHT_PRESSED = false;
let SPACEBAR_PRESSED = false;
let UP_CHANGE = false;
let DOWN_CHANGE = false;
let LEFT_CHANGE = false;
let RIGHT_CHANGE = false;
let SPACEBAR_CHANGE = false;

function getInput() {
    let input = {
        up: false,
        down: false,
        left: false,
        right: false,
        shoot: false,
    };
    if (SPACEBAR_PRESSED) {
        input.shoot = true;
    }
    if (UP_PRESSED) {
        input.up = true;
    }
    else if (DOWN_PRESSED) {
        input.down = true;
    }
    else if (LEFT_PRESSED) {
        input.left = true;
    }
    else if (RIGHT_PRESSED) {
        input.right = true;
    }
    return input;
}

// game loop
const loop = () => {
    game.tick(getInput());
    draw();
    requestAnimationFrame(loop);
    console.log(
        (Math.round(game.player_angle() * 100) / 100).toFixed(2) + ' ' 
    + (Math.round(game.player_speed() * 100) / 100).toFixed(2) + ' ' 
    + (Math.round(game.player_x_speed() * 100) / 100).toFixed(2) + ' ' 
    + (Math.round(game.player_y_speed() * 100) / 100).toFixed(2))
}

document.addEventListener('keydown', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        SPACEBAR_PRESSED = true;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        UP_PRESSED = true;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        DOWN_PRESSED= true;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        LEFT_PRESSED = true;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RIGHT_PRESSED = true;
    }
});

document.addEventListener('keyup', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        SPACEBAR_PRESSED = false;
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        UP_PRESSED = false;
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        DOWN_PRESSED = false;
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        LEFT_PRESSED = false;
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        RIGHT_PRESSED = false;
    }
});

// start
requestAnimationFrame(loop);
