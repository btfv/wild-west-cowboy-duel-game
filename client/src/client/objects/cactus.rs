use macroquad::prelude::*;

pub struct Cactus {
    pub x: f32,
    pub y: f32,
}

impl Cactus {
    pub fn draw(&self) {
        let (x, y) = (self.x, self.y);
        let trunk = Color::from_rgba(60, 120, 40, 255);
        let dark = Color::from_rgba(40, 90, 25, 255);
        draw_rectangle(x + 4.0, y, 6.0, 30.0, trunk);
        draw_rectangle(x + 5.0, y, 4.0, 30.0, dark);
        draw_rectangle(x, y + 8.0, 4.0, 6.0, trunk);
        draw_rectangle(x, y + 2.0, 4.0, 6.0, trunk);
        draw_rectangle(x, y + 2.0, 6.0, 4.0, trunk);
        draw_rectangle(x + 10.0, y + 10.0, 4.0, 6.0, trunk);
        draw_rectangle(x + 10.0, y + 4.0, 4.0, 6.0, trunk);
        draw_rectangle(x + 8.0, y + 4.0, 6.0, 4.0, trunk);
    }
}
