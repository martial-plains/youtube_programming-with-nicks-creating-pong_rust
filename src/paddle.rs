use std::ffi::c_int;

use raylib::ffi::{DrawRectangleRounded, GetScreenHeight, IsKeyDown, KeyboardKey, Rectangle};

#[derive(Debug, Default)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: c_int,
}

impl Paddle {
    pub fn limit_movement(&mut self) {
        if self.y <= 0.0 {
            self.y = 0.0;
        }

        unsafe {
            if self.y + self.height >= GetScreenHeight() as f32 {
                self.y = GetScreenHeight() as f32 - self.height;
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            DrawRectangleRounded(
                Rectangle {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                },
                0.8,
                0,
                raylib::color::Color::WHITE.into(),
            )
        }
    }

    pub fn update(&mut self) {
        if unsafe { IsKeyDown(KeyboardKey::KEY_UP as i32) } {
            self.y -= self.speed as f32;
        }

        if unsafe { IsKeyDown(KeyboardKey::KEY_DOWN as i32) } {
            self.y += self.speed as f32;
        }

        self.limit_movement();
    }
}

#[derive(Debug, Default)]
pub struct CpuPaddle(pub Paddle);

impl CpuPaddle {
    pub fn update(&mut self, ball_y: c_int) {
        if self.0.y + self.0.height / 2.0 > ball_y as f32 {
            self.0.y -= self.0.speed as f32;
        }

        if self.0.y + self.0.height / 2.0 <= ball_y as f32 {
            self.0.y += self.0.speed as f32;
        }

        self.0.limit_movement();
    }
}
