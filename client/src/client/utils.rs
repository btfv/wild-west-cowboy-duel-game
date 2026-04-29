pub fn make_id() -> String {
    format!("{}", (macroquad::miniquad::date::now() * 1000.0) as u64)
}

pub fn make_color() -> (u8, u8, u8) {
    let t = macroquad::miniquad::date::now();
    let r = ((t * 73.0) as u64 % 200 + 55) as u8;
    let g = ((t * 137.0) as u64 % 200 + 55) as u8;
    let b = ((t * 211.0) as u64 % 200 + 55) as u8;
    (r, g, b)
}

pub fn make_room_id() -> String {
    let t = (macroquad::miniquad::date::now() * 1000.0) as u64;
    format!("{:x}", t ^ (t >> 16))
}

pub fn join_url(origin: &str, room_id: &str) -> String {
    format!("{}?room={}", origin, room_id)
}
