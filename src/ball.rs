use std::ffi::c_int;

use raylib::ffi::{DrawCircle, GetRandomValue, GetScreenHeight, GetScreenWidth};

use crate::{colors::YELLOW, CPU_SCORE, PLAYER_SCORE};

#[derive(Debug, Default)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub speed_x: c_int,
    pub speed_y: c_int,
    pub radius: f32,
}

impl Ball {
    pub fn draw(&self) {
        unsafe { DrawCircle(self.x as c_int, self.y as c_int, self.radius, YELLOW) };
    }

    pub fn update(&mut self) {
        self.x += self.speed_x as f32;
        self.y += self.speed_y as f32;

        if self.y + self.radius > unsafe { GetScreenHeight() as f32 } || self.y - self.radius <= 0.0
        {
            self.speed_y *= -1;
        }

        // CPU wins
        unsafe {
            if self.x + self.radius >= GetScreenWidth() as f32 {
                CPU_SCORE += 1;
                self.reset_ball();
            }
        }

        if self.x - self.radius <= 0.0 {
            unsafe {
                PLAYER_SCORE += 1;
            }
            self.reset_ball()
        }
    }

    pub fn reset_ball(&mut self) {
        self.x = unsafe { (GetScreenWidth() / 2) as f32 };
        self.y = unsafe { (GetScreenHeight() / 2) as f32 };

        let speed_choices = [-1, 1];
        self.speed_x *= unsafe { speed_choices[GetRandomValue(0, 1) as usize] };
        self.speed_y *= unsafe { speed_choices[GetRandomValue(0, 1) as usize] };
    }
}
