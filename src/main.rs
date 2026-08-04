use macroquad::prelude::*;

const SCREEN_W: f32 = 900.0;
const SCREEN_H: f32 = 700.0;
const PADDLE_W: f32 = 140.0;
const PADDLE_H: f32 = 16.0;
const PADDLE_Y: f32 = SCREEN_H - 70.0;
const PADDLE_SPEED: f32 = 640.0; // pixels per second

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

    loop {
        // update
        let dt = get_frame_time();

        let mut dir = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dir -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dir += 1.0;
        }

        paddle_x += dir * PADDLE_SPEED * dt;
        paddle_x = paddle_x.clamp(PADDLE_W / 2.0, SCREEN_W - PADDLE_W / 2.0);

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        draw_rectangle(
            paddle_x - PADDLE_W / 2.0,
            PADDLE_Y,
            PADDLE_W,
            PADDLE_H,
            WHITE,
        );

        draw_text(
            &format!("dt {:.4}s   fps {}", dt, get_fps()),
            20.0,
            34.0,
            24.0,
            GRAY,
        );

        next_frame().await
    }
}
