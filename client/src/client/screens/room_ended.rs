use crate::client::events::GameEvent;
use crate::client::render::{draw_centered, draw_link};
use crate::client::screens::{Screen, ScreenContext};
use macroquad::prelude::*;

pub struct RoomEndedScreen {}

impl Screen for RoomEndedScreen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent> {
        draw_centered(
            "Game already ended.",
            ctx.sh / 2.0 - 30.0,
            10.0,
            GRAY,
            ctx.font,
        );
        if draw_link(
            "New game",
            ctx.sw / 2.0,
            ctx.sh / 2.0 + 20.0,
            12.0,
            ctx.ox,
            ctx.oy,
            ctx.scale,
            ctx.font,
        ) {
            return vec![GameEvent::OpenBaseUrl];
        }
        vec![]
    }
}
