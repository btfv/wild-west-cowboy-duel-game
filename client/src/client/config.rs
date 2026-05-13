use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub size: f32,
    pub speed: f32,
    pub strike_cooldown: f32,
    pub bullet_speed: f32,
    pub obj_r: f32,
    pub server_time: f64,
}
