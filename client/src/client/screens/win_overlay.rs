use macroquad::prelude::*;
use crate::client::constants::GAME_H;
use crate::client::render::draw_centered;
use crate::client::screens::ScreenContext;

pub struct WinOverlay {
    pub label: String,
    expires_at: f64,
}

impl WinOverlay {
    pub fn new(label: String, local_now: f64) -> Self {
        Self { label, expires_at: local_now + 3.0 }
    }

    pub fn expired(&self, local_now: f64) -> bool {
        local_now >= self.expires_at
    }

    pub fn draw(&self, ctx: &ScreenContext) {
        draw_centered(&self.label, GAME_H / 2.0, 16.0, WHITE, ctx.font);
    }
}
