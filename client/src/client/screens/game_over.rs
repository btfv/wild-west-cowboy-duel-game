use crate::client::constants::{GAME_H, GAME_W};
use crate::client::events::GameEvent;
use crate::client::render::{draw_centered, draw_link};
use crate::client::screens::{Screen, ScreenContext};
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct GameOverScreen {
    pub winner_id: String,
    pub scores: HashMap<String, u32>,
    pub my_id: String,
}

impl Screen for GameOverScreen {
    fn draw(&mut self, ctx: &ScreenContext) -> Vec<GameEvent> {
        draw_rectangle(
            0.0,
            GAME_H / 2.0 - 100.0,
            GAME_W,
            140.0,
            Color::from_rgba(0, 0, 0, 160),
        );

        let is_winner = self.winner_id == self.my_id;
        let win_color = if is_winner {
            Color::from_rgba(255, 230, 60, 255)
        } else {
            Color::from_rgba(220, 80, 80, 255)
        };
        draw_centered(
            if is_winner { "You win!" } else { "Opp wins!" },
            GAME_H / 2.0 - 80.0,
            16.0,
            win_color,
            ctx.font,
        );

        let mut sorted: Vec<(&String, &u32)> = self.scores.iter().collect();
        sorted.sort_by_key(|(pid, _)| if *pid == &self.my_id { 0 } else { 1 });
        let mut sy = GAME_H / 2.0 - 40.0;
        for (pid, score) in &sorted {
            let label = if *pid == &self.my_id {
                format!("You: {score}")
            } else {
                format!("Opp: {score}")
            };
            draw_centered(&label, sy, 12.0, WHITE, ctx.font);
            sy += 20.0;
        }

        if draw_link(
            "New game",
            GAME_W / 2.0,
            sy + 24.0,
            10.0,
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
