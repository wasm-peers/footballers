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

// ==== create game instance and initialize consts ====

const canvas = document.getElementById('canvas');
canvas.setAttribute('width', (2 * 30.0 + 500.0 + 2 * 30.0).toString())
canvas.setAttribute('height', (2 * 30.0 + 300.0).toString())
let ctx = canvas.getContext('2d');
ctx.textAlign = 'center'
ctx.textBaseline = 'middle'

// ==== drawing =====

export function drawStadium(STADIUM_WIDTH, STADIUM_HEIGHT) {
    ctx.fillStyle = STADIUM_COLOR;
    ctx.fillRect(0, 0, STADIUM_WIDTH, STADIUM_HEIGHT);
}

export function drawPitch(
    edges,
    PITCH_LEFT_LINE,
    PITCH_RIGHT_LINE,
    PITCH_TOP_LINE,
    PITCH_BOTTOM_LINE,
    PITCH_LINE_WIDTH,
    STADIUM_WIDTH,
    STADIUM_HEIGHT,
    GOAL_BREADTH,
) {
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

export function drawGoals(goal_posts) {
    goal_posts.forEach(goal => {
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

export function drawScore(score, STADIUM_WIDTH, PITCH_TOP_LINE) {
    // let score = game.get_score();
    ctx.font = 'bold 30px arial';
    ctx.fillStyle = PITCH_LINE_COLOR;
    ctx.fillText(score.red_score + " - " + score.blue_score, STADIUM_WIDTH / 2.0, PITCH_TOP_LINE / 2.0);
}

export function drawPlayers(players) {
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

export function drawBall(ball) {
    // let ball = game.get_ball_entity();
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

export function drawRedScored(STADIUM_WIDTH, STADIUM_HEIGHT) {
    ctx.font = 'bold 42px arial';
    ctx.fillStyle = RED_PLAYER_COLOR;
    ctx.fillText("Red Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.strokeText("Red Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
}

export function drawBlueScored(STADIUM_WIDTH, STADIUM_HEIGHT) {
    ctx.font = 'bold 42px arial';
    ctx.fillStyle = BLUE_PLAYER_COLOR;
    ctx.fillText("Blue Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.strokeText("Blue Scores!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0);
}

export function drawGameEnded(score, STADIUM_WIDTH, STADIUM_HEIGHT) {
    let half_text_height = 21;
    ctx.font = 'bold 42px arial';
    ctx.strokeStyle = OUTLINE_COLOR;
    // let score = game.get_score();
    if (score.red_score > score.blue_score) {
        ctx.fillStyle = RED_PLAYER_COLOR;
        ctx.fillText("Red Won!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 - half_text_height);
        ctx.strokeText("Red Won!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 - half_text_height);
    } else {
        ctx.fillStyle = BLUE_PLAYER_COLOR;
        ctx.fillText("Blue Won!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 - half_text_height);
        ctx.strokeText("Blue Won!", STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 - half_text_height);
    }
    ctx.fillStyle = TEXT_COLOR;
    ctx.fillText(score.red_score + " - " + score.blue_score, STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 + half_text_height);
    ctx.strokeText(score.red_score + " - " + score.blue_score, STADIUM_WIDTH / 2.0, STADIUM_HEIGHT / 2.0 + half_text_height);
}

export function draw(edges, goal_posts, score, players, ball, red_scored, blue_scored, game_ended) {
    drawStadium();
    drawPitch(edges);
    drawGoals(goal_posts);
    drawScore(score);
    drawPlayers(players);
    drawBall(ball);

    // if (game.get_red_scored()) {
    if (red_scored) {
        drawRedScored();
    }
    // if (game.get_blue_scored()) {
    if (blue_scored) {
        drawBlueScored();
    }
    // if (game.get_game_ended()) {
    if (game_ended) {
        drawGameEnded(score);
    }
}