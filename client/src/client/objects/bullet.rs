use crate::client::constants::GAME_H;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
    pub speed: f32,
    pub spawn_time: f64,
}

impl Bullet {
    pub fn pos(&self, now: f64) -> (f32, f32) {
        let elapsed = (now - self.spawn_time).max(0.0) as f32;
        (self.x, self.y + self.dir * self.speed * elapsed)
    }

    pub fn alive(&self, now: f64) -> bool {
        let (_, y) = self.pos(now);
        y >= 0.0 && y <= GAME_H
    }

    pub fn draw(&self, now: f64) {
        let (bx, by) = self.pos(now);
        let (tip_dy, body_dy, base_dy) = if self.dir > 0.0 {
            (5.0, -2.0, -9.0)
        } else {
            (-8.0, -5.0, 2.0)
        };
        draw_rectangle(
            bx - 2.0,
            by + body_dy,
            4.0,
            7.0,
            Color::from_rgba(220, 220, 225, 255),
        );
        draw_rectangle(
            bx - 1.0,
            by + tip_dy,
            2.0,
            3.0,
            Color::from_rgba(60, 60, 70, 255),
        );
        draw_rectangle(
            bx - 2.0,
            by + base_dy,
            4.0,
            2.0,
            Color::from_rgba(150, 150, 160, 255),
        );
    }
}
