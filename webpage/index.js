import { Game } from "football-game";

// constants
const PITCH_COLOR = '#003300';
const PITCH_LINE_COLOR = '#AAAAAA';
const BALL_COLOR = '#EEEEEE';
const PLAYER_COLOR = '#990000';

let game = Game.new();
const WIDTH = game.width();
const HEIGHT = game.height();
const PITCH_LINE_WIDTH = game.pitch_line_width();

// initialization
var ctx = document.getElementById('canvas');
ctx.setAttribute('width', WIDTH)
ctx.setAttribute('height', HEIGHT)
var ctx = ctx.getContext('2d');

// logic
function tick() {
    game.tick();
}

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

// game loop
const loop = () => {
    tick();
    draw();
    requestAnimationFrame(loop);
}

// events
document.addEventListener('keydown', (event) => {
    let keyName = event.key;

    if (keyName == 'Spacebar' || keyName == ' ') {
        game.ball_randomize();
    }
    if (keyName == 'w' || keyName == 'ArrowUp') {
        game.player_accelerate_up();
    }
    else if (keyName == 's' || keyName == 'ArrowDown') {
        game.player_accelerate_down();
    }
    else if (keyName == 'a' || keyName == 'ArrowLeft') {
        game.player_accelerate_left();
    }
    else if (keyName == 'd' || keyName == 'ArrowRight') {
        game.player_accelerate_right();
    }
    else {
        game.player_decelerate();
    }
});

// start
requestAnimationFrame(loop);
