import { Game } from "football-game";

// function for debugging and printing things from rust
export function log(text) {
    return console.log(text);
}
export function alert(text) {
    return alert(text);
}

let game = Game.new();
let edges = game.get_edge_entities();
let goals = game.get_goal_entities();

// constants
const STADIUM_WIDTH = game.get_stadium_width();
const STADIUM_HEIGHT = game.get_stadium_height();

const PITCH_COLOR = '#619F5E';
const PITCH_LINE_COLOR = '#C7E6BD';
const PITCH_LINE_WIDTH = game.get_pitch_line_width();
const BALL_COLOR = '#EEEEEE';
const RED_PLAYER_COLOR = '#E56E56';
const BLUE_PLAYER_COLOR = '#5689E5';
const OUTLINE_COLOR = '#000000';
const OUTLINE_WIDTH = 2;
const STADIUM_COLOR = '#AAAAAA';

const PITCH_LEFT_LINE = game.pitch_left_line();
const PITCH_RIGHT_LINE = game.pitch_right_line();
const PITCH_TOP_LINE = game.pitch_top_line();
const PITCH_BOTTOM_LINE = game.pitch_bottom_line();

const GOAL_BREADTH = game.get_goal_breadth();

// initialization
var ctx = document.getElementById('canvas');
ctx.setAttribute('width', STADIUM_WIDTH)
ctx.setAttribute('height', STADIUM_HEIGHT)
var ctx = ctx.getContext('2d');

function drawStadium() {
    // gray stadium
    ctx.fillStyle = STADIUM_COLOR;
    ctx.fillRect(0, 0, STADIUM_WIDTH, STADIUM_HEIGHT);
}

function drawPitch() {
    // green field
    ctx.fillStyle = PITCH_COLOR;
    ctx.fillRect(PITCH_LEFT_LINE, PITCH_TOP_LINE, PITCH_RIGHT_LINE - PITCH_LEFT_LINE, PITCH_BOTTOM_LINE - PITCH_TOP_LINE);

    ctx.lineWidth = PITCH_LINE_WIDTH;
    
    // pitch white lines
    edges.forEach(edge => {
        if (edge.white) {
            ctx.fillStyle = PITCH_LINE_COLOR;
        } else {
            ctx.fillStyle = OUTLINE_COLOR;
        }
        ctx.fillRect(edge.x, edge.y, edge.width, edge.height);
    });

    // goals white lines
    ctx.fillStyle = PITCH_LINE_COLOR;
    ctx.strokeStyle = PITCH_LINE_COLOR;
    ctx.beginPath()
    ctx.moveTo(PITCH_LEFT_LINE, (STADIUM_HEIGHT - GOAL_BREADTH) / 2);
    ctx.lineTo(PITCH_LEFT_LINE, (STADIUM_HEIGHT + GOAL_BREADTH) / 2);
    ctx.stroke()
    ctx.beginPath()
    ctx.moveTo(PITCH_RIGHT_LINE, (STADIUM_HEIGHT - GOAL_BREADTH) / 2);
    ctx.lineTo(PITCH_RIGHT_LINE, (STADIUM_HEIGHT + GOAL_BREADTH) / 2);
    ctx.stroke()

    const halfW = STADIUM_WIDTH / 2;
    const halfH = STADIUM_HEIGHT / 2;
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
    ctx.moveTo(halfW, PITCH_TOP_LINE);
    ctx.lineTo(halfW, PITCH_BOTTOM_LINE);
    ctx.stroke()
}

function drawGoals() {
    goals.forEach(goal => {
        if (goal.red) {
            ctx.fillStyle = RED_PLAYER_COLOR;
        } else {
            ctx.fillStyle = BLUE_PLAYER_COLOR;
        }
        ctx.beginPath();
        ctx.arc(goal.x, goal.y, goal.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
        ctx.closePath();
        ctx.fill();

        ctx.strokeStyle = OUTLINE_COLOR;
        ctx.lineWidth = OUTLINE_WIDTH;
        ctx.beginPath();
        ctx.arc(goal.x, goal.y, goal.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
        ctx.closePath();
        ctx.stroke();
    })
}

function drawPlayers() {
    let players = game.get_player_entities();
    players.forEach(player => {
        if (player.red) {
            ctx.fillStyle = RED_PLAYER_COLOR;
        } else {
            ctx.fillStyle = BLUE_PLAYER_COLOR;
        }
        ctx.beginPath();
        ctx.arc(player.x, player.y, player.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
        ctx.closePath();
        ctx.fill();

        ctx.strokeStyle = OUTLINE_COLOR;
        ctx.lineWidth = OUTLINE_WIDTH;
        ctx.beginPath();
        ctx.arc(player.x, player.y, player.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
        ctx.closePath();
        ctx.stroke();
    })
}

function drawBall() {
    let ball = game.get_ball_entity();
    ctx.fillStyle = BALL_COLOR;
    ctx.beginPath();
    ctx.arc(ball.x, ball.y, ball.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.lineWidth = OUTLINE_WIDTH;
    ctx.beginPath();
    ctx.arc(ball.x, ball.y, ball.radius - OUTLINE_WIDTH / 2, 0, 2 * Math.PI);
    ctx.closePath();
    ctx.stroke();
}

function draw() {
    drawStadium();
    drawPitch();
    drawGoals();
    drawPlayers();
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
