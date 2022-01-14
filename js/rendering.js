import { Game } from "../../../football_game";

// ==== consts ====

const PITCH_COLOR = '#619F5E';
const PITCH_LINE_COLOR = '#C7E6BD';
const BALL_COLOR = '#EEEEEE';
const RED_PLAYER_COLOR = '#E56E56';
const BLUE_PLAYER_COLOR = '#5689E5';
const OUTLINE_COLOR = '#000000';
const OUTLINE_WIDTH = 2;
const STADIUM_COLOR = '#718C5A';
const TEXT_COLOR = '#FFFFFF';

// ==== uninitialized consts

let STADIUM_WIDTH;
let STADIUM_HEIGHT;

let PITCH_LINE_WIDTH;
let GOAL_BREADTH;

let PITCH_LEFT_LINE;
let PITCH_RIGHT_LINE;
let PITCH_TOP_LINE;
let PITCH_BOTTOM_LINE;

// ==== game objects ====

let game;
let edges;
let goals_posts;
let ctx;

// ==== create game instance and initialize consts ====

export function initGame(sessionId, isHost) {
    game = Game.new(sessionId, isHost);
    edges = game.get_edge_entities();
    goals_posts = game.get_goals_post_entities();
    
    STADIUM_WIDTH = game.get_stadium_width();
    STADIUM_HEIGHT = game.get_stadium_height();
    
    PITCH_LINE_WIDTH = game.get_pitch_line_width();
    GOAL_BREADTH = game.get_goal_breadth();
    
    PITCH_LEFT_LINE = game.pitch_left_line();
    PITCH_RIGHT_LINE = game.pitch_right_line();
    PITCH_TOP_LINE = game.pitch_top_line();
    PITCH_BOTTOM_LINE = game.pitch_bottom_line();

    var canvas = document.getElementById('canvas');
    canvas.setAttribute('width', STADIUM_WIDTH)
    canvas.setAttribute('height', STADIUM_HEIGHT)
    ctx = canvas.getContext('2d');

    ctx.textAlign = 'center'
    ctx.textBaseline = 'middle'
    
    game.start();
}

// ==== calculate next game frame ====

export function tick() {
    game.tick();
}

// ==== send messages ====

export function hostSendState() {
    game.hostSendState();
}

export function gamerSendInput() {
    game.gamerSendInput();
}

// ==== drawing =====

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
    goals_posts.forEach(goal => {
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

        // draw number
        ctx.font = 'bold 18px arial';
        ctx.fillStyle = TEXT_COLOR;
        ctx.fillText(player.player_number.toString(10), player.x, player.y);
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

function drawRedScored() {
    ctx.font = 'bold 42px arial';
    ctx.fillStyle = RED_PLAYER_COLOR;
    ctx.fillText("Red Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.strokeText("Red Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
}

function drawBlueScored() {
    ctx.font = 'bold 42px arial';
    ctx.fillStyle = BLUE_PLAYER_COLOR;
    ctx.fillText("Blue Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.strokeText("Blue Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
}

export function draw() {
    drawStadium();
    drawPitch();
    drawGoals();
    drawPlayers();
    drawBall();

    if (game.red_scored()) {
        drawRedScored();
    }
    if (game.blue_scored()) {
        drawBlueScored();
    }
}