use crate::client::constants::GAME_W;
use crate::client::events::GameEvent;
use crate::client::render::{FONT_ATLAS_SIZE, draw_centered, font_scale, hovered_text, txt};
use crate::client::screens::{Screen, ScreenContext};
use macroquad::prelude::*;

pub struct WaitingScreen {
    qr_tex: Option<Texture2D>,
}

impl WaitingScreen {
    pub fn new(url: &str) -> Self {
        let qr_tex = crate::client::render::make_qr_texture(url);
        if let Some(ref t) = qr_tex {
            t.set_filter(FilterMode::Nearest);
        }
        Self { qr_tex }
    }
}

impl Screen for WaitingScreen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent> {
        draw_centered("Waiting for", 20.0, 10.0, WHITE, ctx.font);
        draw_centered("second player...", 35.0, 10.0, WHITE, ctx.font);

        if let Some(ref tex) = self.qr_tex {
            let qr_size = 104.0_f32;
            draw_texture_ex(
                tex,
                GAME_W / 2.0 - qr_size / 2.0,
                50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(qr_size, qr_size)),
                    ..Default::default()
                },
            );
        }

        let dim_label = measure_text(
            "Share link:",
            Some(ctx.font),
            FONT_ATLAS_SIZE,
            font_scale(8.0),
        );
        txt(
            "Share link:",
            GAME_W / 2.0 - dim_label.width / 2.0,
            166.0,
            8.0,
            GRAY,
            ctx.font,
        );

        let dim_copy = measure_text(
            ctx.share_label,
            Some(ctx.font),
            FONT_ATLAS_SIZE,
            font_scale(8.0),
        );
        let cx = GAME_W / 2.0 - dim_copy.width / 2.0;
        let hovered_copy = hovered_text(
            ctx.share_label,
            cx,
            200.0,
            8.0,
            ctx.ox,
            ctx.oy,
            ctx.scale,
            ctx.font,
        );
        txt(
            ctx.share_label,
            cx,
            200.0,
            8.0,
            if hovered_copy { YELLOW } else { WHITE },
            ctx.font,
        );
        if hovered_copy && is_mouse_button_pressed(MouseButton::Left) {
            return vec![GameEvent::CopyRoomLink];
        }

        vec![]
    }
}
