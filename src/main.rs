mod logo;
use crate::logo::{LOGO16, LOGO32, LOGO64};
use macroquad::{miniquad::conf::Icon, prelude::*};

const PADDLE: (f32, f32, Color) = (
    15.,                       /* paddle width */
    150.,                      /* paddle height */
    Color::from_hex(0x007BFF), /* paddle color */
);
const BALL: (f32, Color) = (
    10.,                       /* ball radius */
    Color::from_hex(0xFFFFFF), /* ball color */
);
const WINDOW: (f32, f32) = (800. /* window width */, 600. /* window height */);

#[macroquad::main(window_conf)]
async fn main() {
    let player_position: (f32, f32) = (50. - PADDLE.0, (WINDOW.1 - PADDLE.1) / 2.);
    let opponent_position: (f32, f32) = (WINDOW.0 - 50., player_position.1);
    let ball_position: (f32, f32) = (WINDOW.0 / 2., WINDOW.1 / 2.);

    loop {
        clear_background(GRAY);

        draw_rectangle(
            player_position.0,
            player_position.1,
            PADDLE.0,
            PADDLE.1,
            PADDLE.2,
        );
        draw_rectangle(
            opponent_position.0,
            opponent_position.1,
            PADDLE.0,
            PADDLE.1,
            PADDLE.2,
        );
        draw_circle(ball_position.0, ball_position.1, BALL.0, BALL.1);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_width: WINDOW.0 as i32,
        window_height: WINDOW.1 as i32,
        window_resizable: false,
        window_title: "Pong".to_owned(),
        icon: Some(Icon {
            small: LOGO16,
            medium: LOGO32,
            big: LOGO64,
        }),
        ..Default::default()
    }
}
