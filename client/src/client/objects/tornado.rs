use macroquad::prelude::*;
use crate::client::constants::GAME_W;
use crate::client::renderer::DrawContext;

pub struct Tornado {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub anim: f32,
}

impl Tornado {
    pub fn update(&mut self, dt: f32, obj_r: f32) {
        // left extent: ox + (-3 - 2)*s = self.x - 11*s - 5*s = self.x - 16*s
        // right extent: ox + (24 + 2 + 2)*s = self.x - 11*s + 28*s = self.x + 17*s
        // s = obj_r / 14
        let pad_left  = obj_r * 16.0 / 14.0;
        let pad_right = obj_r * 17.0 / 14.0;
        self.x = (self.x + self.vx * dt).clamp(pad_left, GAME_W - pad_right);
        self.anim = (self.anim + dt * 3.0) % std::f32::consts::TAU;
    }

    pub fn draw(&self, ctx: &DrawContext) {
        // Scale relative to obj_r, same convention as cactus
        let s = ctx.obj_r / 14.0;
        let ox = self.x - 11.0 * s;
        let oy = self.y - 14.0 * s;

        // Tornado silhouette: wide at bottom, narrow at top
        // 5 horizontal bands, each 2px shorter on each side going up
        let sand  = Color::from_rgba(200, 175, 110, 255);
        let dark  = Color::from_rgba(130, 105,  55, 255);
        let light = Color::from_rgba(235, 215, 155, 255);
        let grey  = Color::from_rgba(160, 145,  90, 200);

        // bands from top (wide cloud base) to bottom (narrow tip)
        // each band: (x_offset, width, y_offset, height, color)
        let bands: &[(f32, f32, f32, f32, Color)] = &[
            ( 0.0, 22.0,  0.0, 4.0, sand),   // wide top
            ( 0.0, 22.0,  4.0, 4.0, dark),
            ( 2.0, 18.0,  8.0, 4.0, sand),
            ( 4.0, 14.0, 12.0, 4.0, dark),
            ( 6.0, 10.0, 16.0, 4.0, sand),
            ( 8.0,  6.0, 20.0, 4.0, dark),   // narrow tip
        ];

        for &(bx, bw, by, bh, col) in bands {
            draw_rectangle(ox + bx*s, oy + by*s, bw*s, bh*s, col);
        }

        // shading strip on left edge of each band
        for &(bx, _bw, by, bh, _col) in bands {
            draw_rectangle(ox + bx*s, oy + by*s, 2.0*s, bh*s, dark);
        }

        // highlight strip on right-ish of each band
        for &(bx, bw, by, bh, _col) in bands {
            draw_rectangle(ox + (bx + bw - 4.0)*s, oy + by*s, 2.0*s, bh*s, light);
        }

        // spinning debris dots
        let debris_offsets: &[(f32, f32)] = &[
            (-3.0,  2.0),
            (24.0,  4.0),
            (-2.0, 10.0),
            (23.0, 12.0),
        ];
        let wobble = (self.anim.sin() * 2.0) as i32 as f32;
        for (i, &(dx, dy)) in debris_offsets.iter().enumerate() {
            let w = if i % 2 == 0 { wobble } else { -wobble };
            draw_rectangle(ox + (dx + w)*s, oy + dy*s, 2.0*s, 2.0*s, grey);
        }

        // ground dust puff at the tip
        draw_rectangle(ox +  4.0*s, oy + 24.0*s, 14.0*s, 2.0*s, Color::from_rgba(200, 175, 110, 140));
        draw_rectangle(ox +  6.0*s, oy + 25.0*s, 10.0*s, 2.0*s, Color::from_rgba(200, 175, 110, 100));
        draw_rectangle(ox +  8.0*s, oy + 26.0*s,  6.0*s, 1.0*s, Color::from_rgba(200, 175, 110,  70));
    }
}
