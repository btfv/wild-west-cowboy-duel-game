use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::client::config::ServerConfig;
use crate::client::objects::cowboy::PlayerState;
use crate::client::objects::map_object::MapObjectData;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    Config(ServerConfig),
    State(PlayerState),
    Leave { id: String },
    Bullet { x: f32, y: f32, dir: f32, spawn_time: f64 },
    BulletMod { obj_id: String, bx: f32, by: f32, speed: f32, dir: f32, spawn_time: f64 },
    Scores { scores: HashMap<String, u32> },
    Win { id: String },
    Hit { id: String, x: f32, y: f32 },
    Objects { objects: Vec<MapObjectData> },
    Cactuses { positions: Vec<(f32, f32)> },
    Reset,
    Full,
    Waiting { room_id: String },
    Start,
    GameOver { winner: String, scores: HashMap<String, u32> },
    RoomEnded,
}

unsafe extern "C" {
    fn ws_connect(ptr: *const u8, len: u32);
    fn ws_is_connected() -> i32;
    fn ws_failed() -> i32;
    fn ws_send(ptr: *const u8, len: u32);
    fn ws_recv_len() -> u32;
    fn ws_recv_into(ptr: *mut u8, len: u32);
    fn get_query_param(name_ptr: *const u8, name_len: u32, out_ptr: *mut u8, out_len_ptr: *mut u32);
    fn open_url(ptr: *const u8, len: u32);
    fn copy_to_clipboard(ptr: *const u8, len: u32);
    fn get_page_origin(out_ptr: *mut u8, out_len_ptr: *mut u32);
}

pub fn js_ws_connect(url: &str) { unsafe { ws_connect(url.as_ptr(), url.len() as u32) }; }
pub fn js_ws_connected() -> bool { unsafe { ws_is_connected() == 1 } }
pub fn js_ws_failed() -> bool { unsafe { ws_failed() == 1 } }
pub fn js_ws_send(msg: &str) { unsafe { ws_send(msg.as_ptr(), msg.len() as u32) }; }

pub fn js_ws_try_recv() -> Option<String> {
    let len = unsafe { ws_recv_len() };
    if len == 0 { return None; }
    let mut buf = vec![0u8; len as usize];
    unsafe { ws_recv_into(buf.as_mut_ptr(), len) };
    String::from_utf8(buf).ok()
}

pub fn js_get_query_param(name: &str) -> String {
    let mut buf = vec![0u8; 256];
    let mut len: u32 = buf.len() as u32;
    unsafe { get_query_param(name.as_ptr(), name.len() as u32, buf.as_mut_ptr(), &mut len) };
    String::from_utf8(buf[..len as usize].to_vec()).unwrap_or_default()
}

pub fn js_open_url(url: &str) { unsafe { open_url(url.as_ptr(), url.len() as u32) }; }
pub fn js_copy_to_clipboard(text: &str) { unsafe { copy_to_clipboard(text.as_ptr(), text.len() as u32) }; }

pub fn js_page_origin() -> String {
    let mut buf = vec![0u8; 256];
    let mut len: u32 = buf.len() as u32;
    unsafe { get_page_origin(buf.as_mut_ptr(), &mut len) };
    String::from_utf8(buf[..len as usize].to_vec()).unwrap_or_default()
}

pub fn js_ws_url(room_id: &str, player_id: &str) -> String {
    const BASE: &str = env!("WS_URL");
    format!("{}?room={}&id={}", BASE, room_id, player_id)
}
