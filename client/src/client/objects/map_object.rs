use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ObjKind {
    Slow,
    Fast,
    Cow,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapObjectData {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub kind: ObjKind,
    #[serde(default)]
    pub vx: f32,
}
