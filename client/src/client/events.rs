use crate::client::objects::cowboy::PlayerState;
use crate::client::objects::map_object::MapObjectData;
use std::collections::HashMap;

pub enum GameEvent {
    // player
    PlayerJoined(PlayerState),
    PlayerLeft(String),

    // bullets
    BulletFired {
        x: f32,
        y: f32,
        dir: f32,
        spawn_time: f64,
    },
    BulletModified {
        obj_id: String,
        bx: f32,
        by: f32,
        speed: f32,
        dir: f32,
        spawn_time: f64,
    },
    Hit {
        id: String,
        x: f32,
        y: f32,
    },

    // map objects
    ObjectsSpawned(Vec<MapObjectData>),
    CactusesSpawned(Vec<(f32, f32)>),

    // scoring
    ScoresUpdated(HashMap<String, u32>),
    Win(String),

    // lifecycle
    GameStarted,
    Reset,

    // screen / UI
    ShowWaiting(String),
    ShowServerFull,
    ShowRoomEnded,
    ShowGameOver {
        winner_id: String,
        scores: HashMap<String, u32>,
    },
    OpenBaseUrl,
    CopyRoomLink,
    FreezeRequested,
}
