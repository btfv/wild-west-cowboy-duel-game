use macroquad::prelude::*;
use crate::client::constants::GAME_W;
use crate::client::renderer::DrawContext;

pub struct Tumbleweed {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub anim: f32,
}

impl Tumbleweed {
    pub fn update(&mut self, dt: f32, obj_r: f32) {
        self.x = (self.x + self.vx * dt).clamp(obj_r, GAME_W - obj_r);
        self.anim = (self.anim + dt * 1.4) % std::f32::consts::TAU;
    }

    pub fn draw(&self, ctx: &DrawContext) {
        let r = ctx.obj_r;
        let cx = self.x;
        let cy = self.y;
        let a = self.anim;

        let brown = Color::from_rgba(145, 95,  35, 255);
        let dark  = Color::from_rgba( 85, 52,  14, 255);
        let tan   = Color::from_rgba(195, 155, 75, 255);
        let light = Color::from_rgba(215, 180, 110, 200);

        let dot = r * 0.16; // size of each pixel dot

        // rotate a point around (cx, cy) by angle a
        let rot = |px: f32, py: f32| -> (f32, f32) {
            let (s, c) = a.sin_cos();
            (cx + px * c - py * s, cy + px * s + py * c)
        };

        // --- outer circle: 24 dots evenly spaced on circumference ---
        let ring_r = r * 0.88;
        for i in 0..24usize {
            let angle = (i as f32) * std::f32::consts::TAU / 24.0;
            let (px, py) = rot(angle.cos() * ring_r, angle.sin() * ring_r);
            draw_rectangle(px - dot * 0.5, py - dot * 0.5, dot, dot, brown);
        }

        // --- mid ring: 16 dots ---
        let mid_r = r * 0.58;
        for i in 0..16usize {
            let angle = (i as f32) * std::f32::consts::TAU / 16.0;
            let (px, py) = rot(angle.cos() * mid_r, angle.sin() * mid_r);
            draw_rectangle(px - dot * 0.5, py - dot * 0.5, dot, dot, dark);
        }

        // --- 6 radial spokes from center to outer ring ---
        let spoke_dot = r * 0.13;
        for spoke in 0..6usize {
            let base_angle = (spoke as f32) * std::f32::consts::TAU / 6.0;
            // 5 dots per spoke, stepping outward
            for step in 1..=5usize {
                let t = step as f32 / 5.0;
                let sr = ring_r * t;
                let (px, py) = rot(base_angle.cos() * sr, base_angle.sin() * sr);
                let col = if step % 2 == 0 { dark } else { tan };
                draw_rectangle(px - spoke_dot * 0.5, py - spoke_dot * 0.5, spoke_dot, spoke_dot, col);
            }
        }

        // --- 6 diagonal cross-spokes at 30° offset ---
        for spoke in 0..6usize {
            let base_angle = std::f32::consts::PI / 6.0 + (spoke as f32) * std::f32::consts::TAU / 6.0;
            for step in 1..=4usize {
                let t = step as f32 / 4.0;
                let sr = mid_r * t;
                let (px, py) = rot(base_angle.cos() * sr, base_angle.sin() * sr);
                draw_rectangle(px - spoke_dot * 0.5, py - spoke_dot * 0.5, spoke_dot, spoke_dot, brown);
            }
        }

        // --- center knot ---
        let kd = r * 0.20;
        draw_rectangle(cx - kd * 0.5, cy - kd * 0.5, kd, kd, dark);

        // --- highlight glint (rotates with ball) ---
        let (gx, gy) = rot(-r * 0.38, -r * 0.38);
        draw_rectangle(gx - dot * 0.5, gy - dot * 0.5, dot, dot, light);

        // ground shadow
        draw_rectangle(cx - r * 0.72, cy + r * 0.88, r * 1.44, r * 0.11, Color::from_rgba(0, 0, 0, 50));
    }
}
