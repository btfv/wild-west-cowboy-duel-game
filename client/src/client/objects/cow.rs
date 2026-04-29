use crate::client::renderer::DrawContext;
use macroquad::prelude::*;

const FALLING_DURATION: f64 = 1.0;

#[derive(Clone)]
pub struct Cow {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub hit_at: Option<f64>,
}

impl Cow {
    pub async fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            id,
            x,
            y,
            hit_at: None,
        }
    }

    pub fn hit(&mut self, local_now: f64) {
        self.hit_at = Some(local_now);
    }

    pub fn draw(&self, ctx: &DrawContext) {
        let elapsed = self.hit_at.map(|t| ctx.local_now - t);
        let fallen = matches!(elapsed, Some(e) if e >= FALLING_DURATION);
        let falling = matches!(elapsed, Some(e) if e < FALLING_DURATION);
        draw_cow(self.x, self.y, ctx.obj_r, fallen, falling, 255);
    }
}

fn rect(ox: f32, oy: f32, x: f32, y: f32, w: f32, h: f32, col: Color) {
    draw_rectangle(ox + x, oy + y, w, h, col);
}

fn draw_cow(cx: f32, cy: f32, obj_r: f32, fallen: bool, falling: bool, alpha: u8) {
    let s = obj_r / 14.0;
    // anchor: center of the sprite bounding box
    let ox = cx - 14.0 * s;
    let oy = cy - 14.0 * s;

    let white = Color::from_rgba(240, 235, 225, alpha);
    let black = Color::from_rgba(30, 25, 20, alpha);
    let spot = Color::from_rgba(40, 35, 30, alpha);
    let pink = Color::from_rgba(230, 160, 160, alpha);
    let dark_grey = Color::from_rgba(80, 75, 70, alpha);
    let hoof = Color::from_rgba(50, 40, 30, alpha);

    if fallen {
        // lying on its side — body is a wide horizontal rectangle
        // body
        rect(ox, oy, 2.0 * s, 10.0 * s, 22.0 * s, 8.0 * s, white);
        // spots
        rect(ox, oy, 5.0 * s, 11.0 * s, 5.0 * s, 4.0 * s, spot);
        rect(ox, oy, 14.0 * s, 11.0 * s, 4.0 * s, 3.0 * s, spot);
        // head (left side)
        rect(ox, oy, 0.0 * s, 8.0 * s, 5.0 * s, 6.0 * s, white);
        rect(ox, oy, 0.0 * s, 12.0 * s, 4.0 * s, 3.0 * s, white); // snout
        rect(ox, oy, 0.5 * s, 13.0 * s, 1.5 * s, 1.5 * s, pink); // nostril
        rect(ox, oy, 1.0 * s, 8.5 * s, 1.5 * s, 1.5 * s, black); // eye
        // ear
        rect(ox, oy, 3.0 * s, 7.0 * s, 2.0 * s, 2.0 * s, white);
        rect(ox, oy, 3.5 * s, 7.2 * s, 1.0 * s, 1.0 * s, pink);
        // legs sticking up (4 stumps)
        rect(ox, oy, 6.0 * s, 7.0 * s, 2.0 * s, 3.5 * s, white);
        rect(ox, oy, 6.0 * s, 7.0 * s, 2.0 * s, 1.0 * s, hoof);
        rect(ox, oy, 10.0 * s, 7.0 * s, 2.0 * s, 3.5 * s, white);
        rect(ox, oy, 10.0 * s, 7.0 * s, 2.0 * s, 1.0 * s, hoof);
        rect(ox, oy, 15.0 * s, 7.0 * s, 2.0 * s, 3.5 * s, white);
        rect(ox, oy, 15.0 * s, 7.0 * s, 2.0 * s, 1.0 * s, hoof);
        rect(ox, oy, 19.0 * s, 7.0 * s, 2.0 * s, 3.5 * s, white);
        rect(ox, oy, 19.0 * s, 7.0 * s, 2.0 * s, 1.0 * s, hoof);
        // tail
        rect(ox, oy, 23.0 * s, 11.0 * s, 2.0 * s, 1.5 * s, dark_grey);
        rect(ox, oy, 24.5 * s, 12.0 * s, 1.5 * s, 2.0 * s, dark_grey);
    } else if falling {
        // tipping — body tilted, legs flailing outward
        // body (shifted up-right slightly)
        rect(ox, oy, 5.0 * s, 5.0 * s, 16.0 * s, 10.0 * s, white);
        // spots
        rect(ox, oy, 8.0 * s, 6.0 * s, 4.0 * s, 3.0 * s, spot);
        rect(ox, oy, 15.0 * s, 8.0 * s, 3.0 * s, 3.0 * s, spot);
        // head (top, leaning)
        rect(ox, oy, 2.0 * s, 3.0 * s, 5.0 * s, 5.0 * s, white);
        rect(ox, oy, 1.0 * s, 6.0 * s, 4.0 * s, 3.0 * s, white); // snout
        rect(ox, oy, 1.5 * s, 7.0 * s, 1.5 * s, 1.5 * s, pink);
        rect(ox, oy, 3.5 * s, 3.5 * s, 1.5 * s, 1.5 * s, black); // eye (wide/scared)
        rect(ox, oy, 5.0 * s, 3.5 * s, 1.0 * s, 1.0 * s, black); // second eye dot
        // ear
        rect(ox, oy, 4.0 * s, 1.5 * s, 2.0 * s, 2.5 * s, white);
        rect(ox, oy, 4.5 * s, 1.8 * s, 1.0 * s, 1.2 * s, pink);
        // front legs kicking out left
        rect(ox, oy, 0.0 * s, 10.0 * s, 5.0 * s, 2.0 * s, white);
        rect(ox, oy, 0.0 * s, 10.0 * s, 1.5 * s, 2.0 * s, hoof);
        rect(ox, oy, 0.0 * s, 14.0 * s, 5.0 * s, 2.0 * s, white);
        rect(ox, oy, 0.0 * s, 14.0 * s, 1.5 * s, 2.0 * s, hoof);
        // back legs kicking out right
        rect(ox, oy, 21.0 * s, 8.0 * s, 5.0 * s, 2.0 * s, white);
        rect(ox, oy, 24.5 * s, 8.0 * s, 1.5 * s, 2.0 * s, hoof);
        rect(ox, oy, 21.0 * s, 12.0 * s, 5.0 * s, 2.0 * s, white);
        rect(ox, oy, 24.5 * s, 12.0 * s, 1.5 * s, 2.0 * s, hoof);
        // tail flipping up
        rect(ox, oy, 20.0 * s, 3.0 * s, 1.5 * s, 4.0 * s, dark_grey);
        rect(ox, oy, 21.0 * s, 2.0 * s, 2.0 * s, 1.5 * s, dark_grey);
    } else {
        // idle standing cow, side view
        // legs (4)
        rect(ox, oy, 5.0 * s, 18.0 * s, 2.5 * s, 6.0 * s, white);
        rect(ox, oy, 5.0 * s, 22.5 * s, 2.5 * s, 1.5 * s, hoof);
        rect(ox, oy, 9.0 * s, 18.0 * s, 2.5 * s, 6.0 * s, white);
        rect(ox, oy, 9.0 * s, 22.5 * s, 2.5 * s, 1.5 * s, hoof);
        rect(ox, oy, 15.0 * s, 18.0 * s, 2.5 * s, 6.0 * s, white);
        rect(ox, oy, 15.0 * s, 22.5 * s, 2.5 * s, 1.5 * s, hoof);
        rect(ox, oy, 19.0 * s, 18.0 * s, 2.5 * s, 6.0 * s, white);
        rect(ox, oy, 19.0 * s, 22.5 * s, 2.5 * s, 1.5 * s, hoof);
        // udder
        rect(ox, oy, 10.0 * s, 20.0 * s, 6.0 * s, 3.0 * s, pink);
        // body
        rect(ox, oy, 4.0 * s, 9.0 * s, 18.0 * s, 10.0 * s, white);
        // spots
        rect(ox, oy, 7.0 * s, 10.0 * s, 5.0 * s, 4.0 * s, spot);
        rect(ox, oy, 16.0 * s, 12.0 * s, 4.0 * s, 4.0 * s, spot);
        // neck
        rect(ox, oy, 3.0 * s, 10.0 * s, 4.0 * s, 6.0 * s, white);
        // head
        rect(ox, oy, 0.0 * s, 6.0 * s, 6.0 * s, 7.0 * s, white);
        // snout
        rect(ox, oy, 0.0 * s, 10.0 * s, 4.0 * s, 4.0 * s, white);
        rect(ox, oy, 0.5 * s, 11.0 * s, 1.2 * s, 1.2 * s, pink); // nostril L
        rect(ox, oy, 2.0 * s, 11.0 * s, 1.2 * s, 1.2 * s, pink); // nostril R
        // eye
        rect(ox, oy, 3.5 * s, 7.0 * s, 2.0 * s, 2.0 * s, black);
        rect(ox, oy, 4.2 * s, 7.3 * s, 0.8 * s, 0.8 * s, white); // gleam
        // ear
        rect(ox, oy, 4.5 * s, 4.5 * s, 2.5 * s, 3.0 * s, white);
        rect(ox, oy, 5.0 * s, 5.0 * s, 1.2 * s, 1.8 * s, pink);
        // horn
        rect(
            ox,
            oy,
            3.5 * s,
            4.0 * s,
            1.5 * s,
            3.0 * s,
            Color::from_rgba(210, 190, 130, alpha),
        );
        // tail
        rect(ox, oy, 21.5 * s, 10.0 * s, 1.5 * s, 5.0 * s, dark_grey);
        rect(ox, oy, 22.0 * s, 14.5 * s, 2.0 * s, 2.5 * s, dark_grey);
    }
}
