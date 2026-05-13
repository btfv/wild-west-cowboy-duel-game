use crate::client::events::GameEvent;
use crate::client::render::draw_centered;
use crate::client::screens::{Screen, ScreenContext};
use macroquad::prelude::*;

pub struct ServerFullScreen;

impl Screen for ServerFullScreen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent> {
        draw_centered("Room is full!", ctx.sh / 2.0, 12.0, RED, ctx.font);
        vec![]
    }
}
