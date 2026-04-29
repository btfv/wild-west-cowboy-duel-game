use macroquad::prelude::*;
use crate::client::events::GameEvent;
use crate::client::render::draw_centered;
use crate::client::screens::{Screen, ScreenContext};

pub struct ConnectingScreen;

impl Screen for ConnectingScreen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent> {
        draw_centered("Connecting...", ctx.sh / 2.0, 12.0, WHITE, ctx.font);
        vec![]
    }
}
