pub mod connecting;
pub mod game_over;
pub mod room_ended;
pub mod server_full;
pub mod waiting;
pub mod win_overlay;

use crate::client::events::GameEvent;

pub struct ScreenContext<'a> {
    pub font: &'a macroquad::text::Font,
    pub sw: f32,
    pub sh: f32,
    pub ox: f32,
    pub oy: f32,
    pub scale: f32,
    pub share_label: &'static str,
}

pub trait Screen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent>;
}
