use crate::client::constants::{GAME_H, GAME_W};
use macroquad::prelude::*;
use qrcode::{Color as QrColor, QrCode};

// Glyphs are rasterized at FONT_ATLAS_SIZE and scaled down for crispness.
pub const FONT_ATLAS_SIZE: u16 = 128;

pub struct RenderCtx {
    pub font: macroquad::text::Font,
    pub bg_seed: u32,
}

pub fn font_scale(size: f32) -> f32 {
    size / FONT_ATLAS_SIZE as f32
}

pub fn text_params<'a>(size: f32, color: Color, font: &'a Font) -> TextParams<'a> {
    TextParams {
        font: Some(font),
        font_size: FONT_ATLAS_SIZE,
        font_scale: font_scale(size),
        color,
        ..Default::default()
    }
}

pub fn draw_centered(text: &str, y: f32, size: f32, color: Color, font: &Font) {
    let dim = measure_text(text, Some(font), FONT_ATLAS_SIZE, font_scale(size));
    draw_text_ex(
        text,
        GAME_W / 2.0 - dim.width / 2.0,
        y,
        text_params(size, color, font),
    );
}

pub fn hovered_text(
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    ox: f32,
    oy: f32,
    scale: f32,
    font: &Font,
) -> bool {
    let dim = measure_text(text, Some(font), FONT_ATLAS_SIZE, font_scale(size));
    let mx = (mouse_position().0 - ox) / scale;
    let my = (mouse_position().1 - oy) / scale;
    mx >= x && mx <= x + dim.width && my >= y - dim.height && my <= y
}

pub fn txt(text: &str, x: f32, y: f32, size: f32, color: Color, font: &Font) {
    draw_text_ex(text, x, y, text_params(size, color, font));
}

// Draws a centered clickable link; returns true if clicked.
pub fn draw_link(
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    ox: f32,
    oy: f32,
    scale: f32,
    font: &Font,
) -> bool {
    let dim = measure_text(text, Some(font), FONT_ATLAS_SIZE, font_scale(size));
    let lx = x - dim.width / 2.0;
    let hovered = hovered_text(text, lx, y, size, ox, oy, scale, font);
    txt(
        text,
        lx,
        y,
        size,
        if hovered {
            YELLOW
        } else {
            Color::from_rgba(100, 180, 255, 255)
        },
        font,
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn set_game_camera(ox: f32, oy: f32, sw: f32, sh: f32, scale: f32) {
    let mut cam =
        Camera2D::from_display_rect(Rect::new(-ox / scale, -oy / scale, sw / scale, sh / scale));
    cam.zoom.y = -cam.zoom.y;
    set_camera(&cam);
}

pub fn make_qr_texture(url: &str) -> Option<Texture2D> {
    let code = QrCode::new(url.as_bytes()).ok()?;
    let modules = code.width();
    let scale = 4usize;
    let size = modules * scale;
    let pixels: Vec<u8> = (0..size * size)
        .flat_map(|i| {
            let (y, x) = (i / size, i % size);
            let dark = code[(y / scale, x / scale)] == QrColor::Dark;
            let v = if dark { 0u8 } else { 255u8 };
            [v, v, v, 255u8]
        })
        .collect();
    Some(Texture2D::from_rgba8(size as u16, size as u16, &pixels))
}

fn lcg(s: u32) -> u32 {
    s.wrapping_mul(2654435761).wrapping_add(1013904223)
}

pub fn draw_background(seed: u32) {
    let sky_bands: &[(f32, u8, u8, u8)] = &[
        (0.00, 210, 140, 60),
        (0.10, 220, 160, 70),
        (0.20, 230, 180, 90),
        (0.32, 240, 200, 110),
        (0.44, 235, 195, 120),
        (0.55, 220, 175, 100),
        (0.65, 195, 155, 80),
        (0.75, 180, 140, 65),
        (0.85, 165, 125, 55),
        (1.00, 150, 110, 45),
    ];
    for w in sky_bands.windows(2) {
        let (y0, r0, g0, b0) = w[0];
        let (y1, r1, g1, b1) = w[1];
        let py0 = (y0 * GAME_H) as i32;
        let py1 = (y1 * GAME_H) as i32;
        for row in py0..py1 {
            let t = (row - py0) as f32 / (py1 - py0).max(1) as f32;
            let r = (r0 as f32 + (r1 as f32 - r0 as f32) * t) as u8;
            let g = (g0 as f32 + (g1 as f32 - g0 as f32) * t) as u8;
            let b = (b0 as f32 + (b1 as f32 - b0 as f32) * t) as u8;
            draw_line(
                0.0,
                row as f32,
                GAME_W,
                row as f32,
                1.0,
                Color::from_rgba(r, g, b, 255),
            );
        }
    }

    for col in 0..16i32 {
        let alpha = ((16 - col) as f32 / 16.0 * 80.0) as u8;
        draw_line(
            col as f32,
            0.0,
            col as f32,
            GAME_H,
            1.0,
            Color::from_rgba(0, 0, 0, alpha),
        );
        draw_line(
            GAME_W - col as f32,
            0.0,
            GAME_W - col as f32,
            GAME_H,
            1.0,
            Color::from_rgba(0, 0, 0, alpha),
        );
    }

    let cols = (GAME_W / 8.0) as i32 + 1;
    let rows = (GAME_H / 8.0) as i32 + 1;
    for row in 0..rows {
        for col in 0..cols {
            let tx = col as f32 / cols as f32;
            let ty = row as f32 / rows as f32;
            let cell_hash = lcg(lcg(seed ^ (row as u32 * 256 + col as u32)));
            let jitter = (cell_hash % 12) as i16 - 6;
            let r_add = ((1.0 - tx) * (1.0 - ty) * 18.0) as i16 + jitter;
            let b_add = (tx * ty * 14.0) as i16 - jitter / 2;
            let dark = (ty * 20.0) as i16;
            let r = (r_add - dark).clamp(0, 35) as u8;
            let b = (b_add - dark / 2).clamp(0, 25) as u8;
            if r > 0 || b > 0 {
                draw_rectangle(
                    col as f32 * 8.0,
                    row as f32 * 8.0,
                    8.0,
                    8.0,
                    Color::from_rgba(r, 0, b, 30),
                );
            }
        }
    }

    let sx = GAME_W * 0.8;
    let sy = GAME_H * 0.08;
    let sr = 10.0_f32;
    let ray_color = Color::from_rgba(255, 220, 60, 200);
    for &(dx, dy) in &[
        (0.0f32, -1.0),
        (0.0, 1.0),
        (-1.0, 0.0),
        (1.0, 0.0),
        (-0.707, -0.707),
        (0.707, -0.707),
        (-0.707, 0.707),
        (0.707, 0.707),
    ] {
        let gap = sr + 3.0;
        draw_line(
            sx + dx * gap,
            sy + dy * gap,
            sx + dx * (gap + 6.0),
            sy + dy * (gap + 6.0),
            2.0,
            ray_color,
        );
    }
    draw_rectangle(
        sx - sr,
        sy - sr,
        sr * 2.0,
        sr * 2.0,
        Color::from_rgba(255, 230, 80, 255),
    );
    for &(cx, cy) in &[
        (sx - sr, sy - sr),
        (sx + sr - 2.0, sy - sr),
        (sx - sr, sy + sr - 2.0),
        (sx + sr - 2.0, sy + sr - 2.0),
    ] {
        draw_rectangle(cx, cy, 2.0, 2.0, Color::from_rgba(220, 170, 80, 255));
    }

    let ground_y = GAME_H * 0.5;
    draw_rectangle(
        60.0,
        ground_y + 2.0,
        10.0,
        5.0,
        Color::from_rgba(90, 60, 20, 255),
    );
    draw_rectangle(
        150.0,
        ground_y + 2.0,
        14.0,
        4.0,
        Color::from_rgba(85, 55, 18, 255),
    );
    draw_rectangle(
        200.0,
        ground_y + 3.0,
        8.0,
        3.0,
        Color::from_rgba(95, 62, 22, 255),
    );
}
