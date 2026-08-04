use macroquad::prelude::*;

const SCREEN_W: f32 = 900.0;
const SCREEN_H: f32 = 700.0;
const PADDLE_W: f32 = 140.0;
const PADDLE_H: f32 = 16.0;
const PADDLE_Y: f32 = SCREEN_H - 70.0;
const PADDLE_SPEED: f32 = 640.0; // pixels per second
const BALL_R: f32 = 9.0;
const BALL_SPEED: f32 = 380.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rustacean Breakout".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paddle_x = SCREEN_W / 2.0; // center of the paddle, paddle starting position
    let mut ball_pos = vec2(SCREEN_W / 2.0, PADDLE_Y - BALL_R - 2.0);
    let mut ball_vel = Vec2::ZERO;
    let mut launched = false;

    loop {
        // update
        let dt = get_frame_time().min(0.05);

        // Paddle
        let mut dir = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dir += 1.0;
        }

        paddle_x += dir * PADDLE_SPEED * dt;
        paddle_x = paddle_x.clamp(PADDLE_W / 2.0, SCREEN_W - PADDLE_W / 2.0);

        // Ball
        if !launched {
            // glued the paddle until the player launches the ball
            ball_pos = vec2(paddle_x, PADDLE_Y - BALL_R - 2.0);
            if is_key_pressed(KeyCode::Space) {
                launched = true;
                ball_vel = vec2(0.35, -1.0).normalize() * BALL_SPEED;
            }
        } else {
            ball_pos += ball_vel * dt;

            // left wall
            if ball_pos.x - BALL_R < 0.0 {
                ball_pos.x = BALL_R;
                ball_vel.x = ball_vel.x.abs();
            }

            // right wall
            if ball_pos.x + BALL_R > SCREEN_W {
                ball_pos.x = SCREEN_W - BALL_R;
                ball_vel.x = -ball_vel.x.abs();
            }

            // ceiling
            if ball_pos.y - BALL_R < 0.0 {
                ball_pos.y = BALL_R;
                ball_vel.y = ball_vel.y.abs();
            }

            // fell off the bottom
            if ball_pos.y + BALL_R > SCREEN_H {
                launched = false;
                ball_vel = Vec2::ZERO;
            }
        }

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        draw_rectangle(
            paddle_x - PADDLE_W / 2.0,
            PADDLE_Y,
            PADDLE_W,
            PADDLE_H,
            WHITE,
        );

        draw_circle(ball_pos.x, ball_pos.y, BALL_R, WHITE);

        if !launched {
            draw_text("SPACE to launch", 20.0, 34.0, 24.0, GRAY);
        }

        next_frame().await
    }
}
