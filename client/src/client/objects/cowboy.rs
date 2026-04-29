use crate::client::constants::GAME_H;
use crate::client::render::{FONT_ATLAS_SIZE, font_scale, txt};
use crate::client::renderer::DrawContext;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub x0: f32,
    pub y: f32,
    pub x1: f32,
    pub speed: f32,
    pub start_time: f64,
    pub move_start: f64,
    pub freeze_end: Option<f64>,
    pub frozen_x: Option<f32>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PlayerState {
    pub fn pos(&self, now: f64) -> (f32, f32) {
        if self.freeze_end.map_or(false, |fe| now < fe) {
            return (self.frozen_x.unwrap_or(self.x0), self.y);
        }
        let elapsed = (now - self.start_time).max(0.0) as f32;
        let dist = (self.x1 - self.x0).abs();
        if dist < 0.001 {
            return (self.x0, self.y);
        }
        let cycle = (elapsed * self.speed / dist) % 2.0;
        let progress = if cycle <= 1.0 { cycle } else { 2.0 - cycle };
        (self.x0 + (self.x1 - self.x0) * progress, self.y)
    }

    pub fn is_frozen(&self, now: f64) -> bool {
        self.freeze_end.map_or(false, |fe| now < fe)
    }
}

pub struct Cowboy {
    pub state: PlayerState,
    death_anim: Option<(f64, f32, f32)>,
}

impl Cowboy {
    pub async fn new(state: PlayerState) -> Self {
        Self {
            state,
            death_anim: None,
        }
    }

    pub fn sync(&mut self, new_state: PlayerState) {
        self.state = new_state;
    }

    pub fn die(&mut self, local_now: f64, x: f32, y: f32) {
        self.death_anim = Some((local_now, x, y));
    }

    pub fn reset(&mut self) {
        self.death_anim = None;
    }

    pub fn is_frozen(&self, now: f64) -> bool {
        self.state.is_frozen(now)
    }

    fn render_pos(&self, now: f64) -> (f32, f32) {
        match self.death_anim {
            Some((_, fx, fy)) => (fx, fy),
            None => self.state.pos(now),
        }
    }

    fn death_elapsed(&self, local_now: f64) -> Option<f64> {
        self.death_anim.map(|(t, _, _)| local_now - t)
    }

    pub fn draw(&self, ctx: &DrawContext) {
        let now = ctx.now();
        let (x, y) = self.render_pos(now);
        let death_elapsed = self.death_elapsed(ctx.local_now);
        let alpha = if self.state.is_frozen(now) && self.death_anim.is_none() {
            120u8
        } else {
            255u8
        };
        let faces_down = y < GAME_H / 2.0;
        let color = Color::from_rgba(self.state.r, self.state.g, self.state.b, alpha);

        match death_elapsed {
            Some(e) if e < 0.5 => draw_cowboy_falling(x, y, ctx.size, color, alpha),
            Some(_) => draw_cowboy_fallen(x, y, ctx.size, color, alpha),
            None => draw_cowboy_idle(x, y, ctx.size, color, alpha, faces_down),
        }
    }

    pub fn draw_cooldown_bar(&self, ctx: &DrawContext) {
        let now = ctx.now();
        let moving_for = (now - self.state.move_start).max(0.0) as f32;
        if moving_for >= ctx.strike_cooldown {
            return;
        }
        let (x, y) = self.render_pos(now);
        let bar_y = if y > GAME_H / 2.0 {
            y - 8.0
        } else {
            y + ctx.size + 5.0
        };
        let fill = (moving_for / ctx.strike_cooldown).min(1.0);
        draw_rectangle(x, bar_y, ctx.size, 3.0, Color::from_rgba(60, 60, 60, 180));
        draw_rectangle(
            x,
            bar_y,
            ctx.size * fill,
            3.0,
            Color::from_rgba(255, 200, 50, 220),
        );
    }

    pub fn draw_score(&self, score: u32, is_me: bool, ctx: &DrawContext) {
        let now = ctx.now();
        let (x, y) = self.render_pos(now);
        let label = if is_me {
            format!("You:{score}")
        } else {
            format!("Opp:{score}")
        };
        let dim = measure_text(&label, Some(ctx.font), FONT_ATLAS_SIZE, font_scale(8.0));
        let label_y = if y > GAME_H / 2.0 {
            y - 12.0
        } else {
            y + ctx.size + 22.0
        };
        txt(
            &label,
            x + ctx.size / 2.0 - dim.width / 2.0,
            label_y,
            8.0,
            Color::from_rgba(255, 230, 80, 255),
            ctx.font,
        );
    }
}

// s = size/32 — all coords in 32-unit space
fn r(ox: f32, oy: f32, x: f32, y: f32, w: f32, h: f32, col: Color) {
    draw_rectangle(ox + x, oy + y, w, h, col);
}

// 32×32 grid, cowboy lies HORIZONTALLY.
// x=0 = feet/boots, x=31 = head+hat end. Body centred at y=12..20.
// Hat sticks RIGHT past the head (same y-band). Gun arm is VERTICAL,
// pointing toward the opponent (down if faces_down, up otherwise).
fn draw_cowboy_idle(bx: f32, by: f32, size: f32, body_col: Color, alpha: u8, faces_down: bool) {
    let s = size / 32.0;

    let p = |x: f32, y: f32, w: f32, h: f32, col: Color| {
        r(bx, by, x * s, y * s, w * s, h * s, col);
    };

    let skin = Color::from_rgba(220, 170, 110, alpha);
    let hat = Color::from_rgba(80, 50, 20, alpha);
    let brim = Color::from_rgba(60, 35, 10, alpha);
    let boot = Color::from_rgba(60, 35, 15, alpha);
    let dark = Color::from_rgba(30, 20, 10, alpha);
    let belt = Color::from_rgba(40, 25, 10, alpha);
    let buckle = Color::from_rgba(210, 175, 50, alpha);
    let gun = Color::from_rgba(50, 50, 50, alpha);
    let barrel = Color::from_rgba(20, 20, 20, alpha);

    // ── boots (x=0..5, two boots split top/bottom) ───────────────────────────
    p(0.0, 9.0, 5.0, 4.0, boot);
    p(0.0, 19.0, 5.0, 4.0, boot);
    p(0.0, 8.0, 3.0, 2.0, dark); // toe tip top
    p(0.0, 22.0, 3.0, 2.0, dark); // toe tip bottom

    // ── legs (x=3..11) ───────────────────────────────────────────────────────
    p(3.0, 11.0, 8.0, 4.0, body_col); // top leg
    p(3.0, 17.0, 8.0, 4.0, body_col); // bottom leg

    // ── torso (x=9..20, y=12..20) ────────────────────────────────────────────
    p(9.0, 12.0, 11.0, 8.0, body_col);
    p(9.0, 15.0, 11.0, 2.0, belt);
    p(12.0, 15.0, 4.0, 2.0, buckle);

    // ── neck (x=19..22, y=14..18) ────────────────────────────────────────────
    p(19.0, 14.0, 3.0, 4.0, skin);

    // ── head (x=20..28, y=12..20) ────────────────────────────────────────────
    p(20.0, 12.0, 8.0, 8.0, skin);
    p(24.0, 13.0, 2.0, 2.0, dark); // eye 1
    p(24.0, 17.0, 2.0, 2.0, dark); // eye 2

    // ── hat: big western hat to the RIGHT of head along x ────────────────────
    // Side-view silhouette: crown is tall in y, brim overhangs top and bottom.
    // Crown: x=27..32, y=9..23  (centred on head y=12..20, taller than head)
    // Brim:  x=25..27, y=7..25  (wide flat overhang on both sides, thin in x)
    // Pinch: dark indent at the very top and bottom of crown
    p(28.0, 12.0, 3.0, 8.0, hat); // crown (narrower in x)
    p(25.0, 7.0, 3.0, 18.0, brim); // brim (unchanged)
    p(28.0, 12.0, 3.0, 1.0, dark); // pinch top
    p(28.0, 19.0, 3.0, 1.0, dark); // pinch bottom

    // ── gun arm: vertical, pointing toward opponent ──────────────────────────
    // faces_down=true  → opponent below → arm/barrel points DOWN (y increases)
    // faces_down=false → opponent above → arm/barrel points UP  (y decreases)
    // arm is centred at x=13..17 on the torso; p() mirrors x automatically.
    if faces_down {
        p(13.0, 20.0, 4.0, 4.0, body_col); // upper arm
        p(13.0, 24.0, 4.0, 2.0, skin); // hand
        p(11.0, 25.0, 4.0, 3.0, gun); // grip
        p(13.0, 25.0, 1.0, 1.0, dark); // trigger
        p(12.0, 27.0, 5.0, 3.0, gun); // cylinder
        p(13.0, 28.0, 1.0, 1.0, dark); // cylinder gap
        p(14.0, 27.0, 2.0, 8.0, barrel); // barrel (long)
    } else {
        p(13.0, 8.0, 4.0, 4.0, body_col); // upper arm (y=8..12, just above body)
        p(13.0, 6.0, 4.0, 2.0, skin); // hand
        p(11.0, 3.0, 4.0, 3.0, gun); // grip
        p(13.0, 4.0, 1.0, 1.0, dark); // trigger
        p(12.0, 1.0, 5.0, 3.0, gun); // cylinder
        p(13.0, 2.0, 1.0, 1.0, dark); // cylinder gap
        p(14.0, -4.0, 2.0, 8.0, barrel); // barrel pointing toward y=0 (up = toward opponent)
    }

    // ── off arm: rests along torso on the opposite side from gun arm ─────────
    // gun arm points down when faces_down, so off-arm hangs at top of torso,
    // and vice versa. Kept short so it doesn't reach the barrel's side.
    if faces_down {
        p(9.0, 9.0, 7.0, 3.0, body_col); // arm along top of torso
        p(14.0, 8.0, 4.0, 3.0, skin); // hand
    } else {
        p(9.0, 20.0, 7.0, 3.0, body_col); // arm along bottom of torso
        p(14.0, 21.0, 4.0, 3.0, skin); // hand
    }
}

fn draw_cowboy_falling(bx: f32, by: f32, size: f32, body_col: Color, alpha: u8) {
    let s = size / 32.0;

    let skin = Color::from_rgba(220, 170, 110, alpha);
    let hat = Color::from_rgba(80, 50, 20, alpha);
    let brim = Color::from_rgba(60, 35, 10, alpha);
    let boot = Color::from_rgba(60, 35, 15, alpha);
    let dark = Color::from_rgba(30, 20, 10, alpha);
    let belt = Color::from_rgba(40, 25, 10, alpha);

    // tilted body — shifted right, leaning
    r(bx, by, 4.0 * s, 8.0 * s, 18.0 * s, 8.0 * s, body_col);
    r(bx, by, 3.0 * s, 16.0 * s, 20.0 * s, 2.0 * s, belt);

    // legs kicking up-left
    r(bx, by, 0.0 * s, 2.0 * s, 6.0 * s, 5.0 * s, body_col);
    r(bx, by, 0.0 * s, 0.0 * s, 5.0 * s, 3.0 * s, boot);
    r(bx, by, 4.0 * s, 4.0 * s, 6.0 * s, 5.0 * s, body_col);
    r(bx, by, 3.0 * s, 6.0 * s, 5.0 * s, 3.0 * s, boot);

    // arms flailing
    r(bx, by, 22.0 * s, 6.0 * s, 8.0 * s, 4.0 * s, body_col);
    r(bx, by, 29.0 * s, 5.0 * s, 3.0 * s, 3.0 * s, skin);
    r(bx, by, 2.0 * s, 14.0 * s, 6.0 * s, 4.0 * s, body_col);
    r(bx, by, 0.0 * s, 13.0 * s, 3.0 * s, 3.0 * s, skin);

    // head leaning
    r(bx, by, 16.0 * s, 18.0 * s, 12.0 * s, 8.0 * s, skin);
    r(bx, by, 18.0 * s, 22.0 * s, 2.0 * s, 2.0 * s, dark);
    r(bx, by, 23.0 * s, 22.0 * s, 2.0 * s, 2.0 * s, dark);

    // hat: same shape as idle, offset to match falling head centre (x=22, y=22)
    r(bx, by, 23.0 * s, 13.0 * s,  3.0 * s, 18.0 * s, brim);
    r(bx, by, 26.0 * s, 18.0 * s,  3.0 * s,  8.0 * s, hat);
    r(bx, by, 26.0 * s, 18.0 * s,  3.0 * s,  1.0 * s, dark);
    r(bx, by, 26.0 * s, 25.0 * s,  3.0 * s,  1.0 * s, dark);
}

fn draw_cowboy_fallen(bx: f32, by: f32, size: f32, body_col: Color, alpha: u8) {
    let s = size / 32.0;

    let skin = Color::from_rgba(220, 170, 110, alpha);
    let hat = Color::from_rgba(80, 50, 20, alpha);
    let brim = Color::from_rgba(60, 35, 10, alpha);
    let boot = Color::from_rgba(60, 35, 15, alpha);
    let dark = Color::from_rgba(30, 20, 10, alpha);
    let belt = Color::from_rgba(40, 25, 10, alpha);

    // body flat horizontal
    r(bx, by, 2.0 * s, 12.0 * s, 22.0 * s, 8.0 * s, body_col);
    r(bx, by, 2.0 * s, 14.0 * s, 22.0 * s, 2.0 * s, belt);

    // legs flat
    r(bx, by, 0.0 * s, 10.0 * s, 4.0 * s, 4.0 * s, boot);
    r(bx, by, 0.0 * s, 16.0 * s, 4.0 * s, 4.0 * s, boot);

    // arms at sides
    r(bx, by, 24.0 * s, 10.0 * s, 4.0 * s, 4.0 * s, body_col);
    r(bx, by, 26.0 * s, 10.0 * s, 4.0 * s, 3.0 * s, skin);
    r(bx, by, 24.0 * s, 16.0 * s, 4.0 * s, 4.0 * s, body_col);
    r(bx, by, 26.0 * s, 17.0 * s, 4.0 * s, 3.0 * s, skin);

    // head on ground (right side)
    r(bx, by, 22.0 * s, 20.0 * s, 10.0 * s, 8.0 * s, skin);
    r(bx, by, 24.0 * s, 23.0 * s, 2.0 * s, 2.0 * s, dark);

    // hat: same shape as idle, offset to match fallen head centre (x=27, y=24)
    r(bx, by, 28.0 * s, 15.0 * s,  3.0 * s, 18.0 * s, brim);
    r(bx, by, 29.0 * s, 20.0 * s,  3.0 * s,  8.0 * s, hat);
    r(bx, by, 29.0 * s, 20.0 * s,  3.0 * s,  1.0 * s, dark);
    r(bx, by, 29.0 * s, 27.0 * s,  3.0 * s,  1.0 * s, dark);
}
