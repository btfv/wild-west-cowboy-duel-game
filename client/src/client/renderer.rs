use macroquad::prelude::*;
use crate::client::world::World;

pub struct DrawContext<'a> {
    pub font: &'a Font,
    pub local_now: f64,
    pub clock_offset: f64,
    pub obj_r: f32,
    pub size: f32,
    pub strike_cooldown: f32,
}

impl<'a> DrawContext<'a> {
    pub fn now(&self) -> f64 {
        self.local_now + self.clock_offset
    }
}

impl World {
    pub fn draw(&self, font: &Font, local_now: f64) {
        let ctx = DrawContext {
            font,
            local_now,
            clock_offset: self.clock_offset,
            obj_r: self.cfg.obj_r,
            size: self.cfg.size,
            strike_cooldown: self.cfg.strike_cooldown,
        };

        for cactus in &self.cactuses       { cactus.draw(); }
        for sc in &self.tumbleweedes      { sc.draw(&ctx); }
        for fc in &self.tornadoes      { fc.draw(&ctx); }
        for cow in &self.cows        { cow.draw(&ctx); }
        for cowboy in self.cowboys.values() { cowboy.draw(&ctx); }
        if let Some(cowboy) = self.cowboys.get(&self.my_id) {
            cowboy.draw_cooldown_bar(&ctx);
        }
        for b in &self.bullets { b.draw(ctx.now()); }
        for (pid, score) in &self.scores {
            if let Some(cowboy) = self.cowboys.get(pid) {
                cowboy.draw_score(*score, pid == &self.my_id, &ctx);
            }
        }
    }
}
