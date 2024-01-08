use std::ffi::{c_int, CString};

use raylib::ffi::{
    BeginDrawing, CheckCollisionCircleRec, ClearBackground, CloseWindow, DrawCircle, DrawLine,
    DrawRectangle, DrawText, EndDrawing, InitWindow, Rectangle, SetTargetFPS, TextFormat, Vector2,
    WindowShouldClose,
};

use crate::{
    ball::Ball,
    colors::{DARK_GREEN, GREEN, LIGHT_GREEN},
    paddle::{CpuPaddle, Paddle},
};

mod ball;
mod colors;
mod paddle;

pub static mut PLAYER_SCORE: c_int = 0;
pub static mut CPU_SCORE: c_int = 0;

fn main() {
    let mut ball = Ball::default();
    let mut player = Paddle::default();
    let mut cpu = CpuPaddle::default();

    println!("Starting the game");
    let screen_width = 1280;
    let screen_height = 800;
    let title = CString::new("My Pong Game!").unwrap();

    unsafe {
        InitWindow(screen_width, screen_height, title.as_ptr());
        SetTargetFPS(60);
    }

    ball.radius = 20.0;
    ball.x = screen_width as f32 / 2.0;
    ball.y = screen_height as f32 / 2.0;
    ball.speed_x = 7;
    ball.speed_y = 7;

    player.width = 25.0;
    player.height = 120.0;
    player.x = 10.0;
    player.y = screen_height as f32 / 2.0 - player.height / 2.0;
    player.speed = 6;

    cpu.0.height = 120.0;
    cpu.0.width = 25.0;
    cpu.0.x = screen_width as f32 - cpu.0.width - 10.0;
    cpu.0.y = screen_height as f32 / 2.0 - cpu.0.height / 2.0;
    cpu.0.speed = 6;

    while !unsafe { WindowShouldClose() } {
        unsafe {
            BeginDrawing();
        }

        // Updating

        ball.update();
        player.update();
        cpu.update(ball.y as c_int);

        // Checking for collisions
        if unsafe {
            CheckCollisionCircleRec(
                Vector2 {
                    x: ball.x,
                    y: ball.y,
                },
                ball.radius,
                Rectangle {
                    x: player.x,
                    y: player.y,
                    width: player.width,
                    height: player.height,
                },
            )
        } {
            ball.speed_x *= -1;
        }

        if unsafe {
            CheckCollisionCircleRec(
                Vector2 {
                    x: ball.x,
                    y: ball.y,
                },
                ball.radius,
                Rectangle {
                    x: cpu.0.x,
                    y: cpu.0.y,
                    width: cpu.0.width,
                    height: cpu.0.height,
                },
            )
        } {
            ball.speed_x *= -1;
        }

        // Drawing
        unsafe {
            ClearBackground(DARK_GREEN);
            DrawRectangle(screen_width / 2, 0, screen_width / 2, screen_height, GREEN);
            DrawCircle(screen_width / 2, screen_height / 2, 150.0, LIGHT_GREEN);
            DrawLine(
                screen_width / 2,
                0,
                screen_width / 2,
                screen_height,
                raylib::color::Color::WHITE.into(),
            );
            ball.draw();
            cpu.0.draw();
            player.draw();

            let player_score_text = CString::new(format!("{PLAYER_SCORE}")).unwrap();
            DrawText(
                TextFormat(player_score_text.as_ptr()),
                3 * screen_width / 4 - 20,
                20,
                80,
                raylib::color::Color::WHITE.into(),
            );

            let cpu_score_text = CString::new(format!("{CPU_SCORE}")).unwrap();
            DrawText(
                TextFormat(cpu_score_text.as_ptr()),
                screen_width / 4 - 20,
                20,
                80,
                raylib::color::Color::WHITE.into(),
            );

            EndDrawing();
        }
    }

    unsafe {
        CloseWindow();
    }
}
