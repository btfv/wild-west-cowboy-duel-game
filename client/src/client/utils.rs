pub fn make_id() -> String {
    format!("{}", (macroquad::miniquad::date::now() * 1000.0) as u64)
}

pub fn make_room_id() -> String {
    let t = (macroquad::miniquad::date::now() * 1000.0) as u64;
    format!("{:x}", t ^ (t >> 16))
}

pub fn join_url(origin: &str, room_id: &str) -> String {
    format!("{}?room={}", origin, room_id)
}
