import { Game } from "football-game";

// function for debugging and printing things from rust
export function log(text) {
    return console.log(text);
}
export function alert(text) {
    return alert(text);
}

// constants
const PITCH_COLOR = '#619F5E';
const PITCH_LINE_COLOR = '#C7E6BD';
const PITCH_LINE_WIDTH = 5;
const BALL_COLOR = '#EEEEEE';
const RED_PLAYER_COLOR = '#E56E56';
const BLUE_PLAYER_COLOR = '#5689E5';
const OUTLINE_COLOR = '#000000';
const OUTLINE_WIDTH = 2;
const STADIUM_COLOR = '#AAAAAA';

let game = Game.new();
const PITCH_LEFT_WALL = game.pitch_left_wall();
const PITCH_RIGHT_WALL = game.pitch_right_wall();
const PITCH_TOP_WALL = game.pitch_top_wall();
const PITCH_BOTTOM_WALL = game.pitch_bottom_wall();
const STADIUM_LEFT_WALL = game.stadium_left_wall();
const STADIUM_RIGHT_WALL = game.stadium_right_wall();
const STADIUM_TOP_WALL = game.stadium_top_wall();
const STADIUM_BOTTOM_WALL = game.stadium_bottom_wall();

const GOAL_LENGTH = game.goal_length();

// initialization
var ctx = document.getElementById('canvas');
ctx.setAttribute('width', STADIUM_RIGHT_WALL)
ctx.setAttribute('height', STADIUM_BOTTOM_WALL)
var ctx = ctx.getContext('2d');

function drawStadium() {
    // gray stadium
    ctx.fillStyle = STADIUM_COLOR;
    ctx.fillRect(STADIUM_LEFT_WALL, STADIUM_TOP_WALL, STADIUM_RIGHT_WALL - STADIUM_LEFT_WALL, STADIUM_BOTTOM_WALL - STADIUM_TOP_WALL);
}

function drawPitch() {
    // green field
    ctx.fillStyle = PITCH_COLOR;
    ctx.fillRect(PITCH_LEFT_WALL, PITCH_TOP_WALL, PITCH_RIGHT_WALL - PITCH_LEFT_WALL, PITCH_BOTTOM_WALL - PITCH_TOP_WALL);

    // set styles for lines
    ctx.lineWidth = PITCH_LINE_WIDTH;
    ctx.fillStyle = PITCH_LINE_COLOR
    ctx.strokeStyle = PITCH_LINE_COLOR;
    const halfW = (STADIUM_RIGHT_WALL - STADIUM_LEFT_WALL) / 2;
    const halfH = (STADIUM_BOTTOM_WALL - STADIUM_TOP_WALL) / 2;

    // boundaries
    ctx.strokeRect(PITCH_LEFT_WALL, PITCH_TOP_WALL, PITCH_RIGHT_WALL - PITCH_LEFT_WALL, PITCH_BOTTOM_WALL - PITCH_TOP_WALL);

    ctx.moveTo(halfW, halfH);

    // middle point
    ctx.beginPath();
    ctx.arc(halfW, halfH, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.fill();

    // middle circle
    ctx.beginPath();
    ctx.arc(halfW, halfH, halfH / 3, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();

    // middle vertical lines
    ctx.beginPath()
    ctx.moveTo(halfW, PITCH_TOP_WALL);
    ctx.lineTo(halfW, PITCH_BOTTOM_WALL);
    ctx.stroke()

    ctx.lineWidth = 1;
}

function drawGoalBlue() {
    
    // higher point
    let y = (STADIUM_BOTTOM_WALL - GOAL_LENGTH) / 2
    ctx.beginPath();
    ctx.arc(PITCH_RIGHT_WALL, y, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.fillStyle = BLUE_PLAYER_COLOR;
    ctx.fill();

    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.lineWidth = OUTLINE_WIDTH;
    ctx.beginPath();
    ctx.arc(PITCH_RIGHT_WALL, y, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();

    // lower point
    y = (STADIUM_BOTTOM_WALL + GOAL_LENGTH) / 2;
    ctx.beginPath();
    ctx.arc(PITCH_RIGHT_WALL, y, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.fillStyle = BLUE_PLAYER_COLOR;
    ctx.fill();

    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.lineWidth = OUTLINE_WIDTH;
    ctx.beginPath();
    ctx.arc(PITCH_RIGHT_WALL, y, 8, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();
}

function drawBall() {
    ctx.fillStyle = BALL_COLOR;
    ctx.beginPath();
    ctx.arc(game.ball_x(), game.ball_y(), game.ball_radius(), 0, 2 * Math.PI, true);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.lineWidth = OUTLINE_WIDTH;
    ctx.beginPath();
    ctx.arc(game.ball_x(), game.ball_y(), game.ball_radius() - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();
}

function drawPlayer() {
    ctx.fillStyle = RED_PLAYER_COLOR;
    ctx.beginPath();
    ctx.arc(game.player_x(), game.player_y(), game.player_radius(), 0, 2 * Math.PI, true);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.lineWidth = OUTLINE_WIDTH;
    ctx.beginPath();
    ctx.arc(game.player_x(), game.player_y(), game.player_radius() - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();
}

function draw() {
    drawStadium();
    drawPitch();
    drawGoalBlue();
    drawBall();
    drawPlayer();
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
const loop = () => {
    game.tick(getInput());
    draw();
    requestAnimationFrame(loop);
    // console.log(
    //     (Math.round(game.player_angle() * 100) / 100).toFixed(2) + ' ' 
    // + (Math.round(game.player_speed() * 100) / 100).toFixed(2) + ' ' 
    // + (Math.round(game.player_x_speed() * 100) / 100).toFixed(2) + ' ' 
    // + (Math.round(game.player_y_speed() * 100) / 100).toFixed(2));
//         game.player_angle() + ' ' 
//         + game.player_speed() + ' ' 
//         + game.player_x_speed() + ' ' 
//         + game.player_y_speed());
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
