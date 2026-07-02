use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

// A single named frame in the atlas. Coordinates are in atlas pixels.
// native_w/native_h are part of the manifest contract for the Designer and
// upcoming sprite-integration work; not all fields are consumed yet.
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Frame {
    // Source rect in the atlas PNG, in pixels.
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    // Native pixel size of the sprite as authored (used for scale reasoning).
    pub native_w: f32,
    pub native_h: f32,
    // Anchor/pivot as a 0..1 fraction of the frame; (0,0) = top-left, (0.5,0.5) = centre.
    #[serde(default = "default_pivot")]
    pub pivot_x: f32,
    #[serde(default = "default_pivot")]
    pub pivot_y: f32,
}

fn default_pivot() -> f32 {
    0.0
}

#[derive(Deserialize)]
struct Manifest {
    frames: HashMap<String, Frame>,
}

pub struct Atlas {
    texture: Texture2D,
    frames: HashMap<String, Frame>,
}

// How to draw a named frame: target top-left, size (square), tint and flip.
pub struct DrawFrame {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
    pub flip_x: bool,
}

impl Atlas {
    // Loads the atlas PNG + JSON manifest from the web root (served from client/public/).
    pub async fn load(png: &str, manifest: &str) -> Result<Self, macroquad::Error> {
        let texture = load_texture(png).await?;
        texture.set_filter(FilterMode::Nearest);

        let bytes = load_file(manifest).await?;
        let manifest: Manifest = serde_json::from_slice(&bytes)
            .map_err(|e| macroquad::Error::UnknownError(e.to_string().leak()))?;

        Ok(Self {
            texture,
            frames: manifest.frames,
        })
    }

    // Used by integration code to decide whether a sprite frame exists yet.
    #[allow(dead_code)]
    pub fn has(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    // Draws a named frame scaled so its frame width maps to `size`, anchored by pivot.
    // Returns false if the frame name is unknown (caller can fall back to hardcoded art).
    pub fn draw(&self, name: &str, d: DrawFrame) -> bool {
        let Some(frame) = self.frames.get(name) else {
            return false;
        };

        let scale = d.size / frame.w;
        let dest_w = frame.w * scale;
        let dest_h = frame.h * scale;
        let dx = d.x - frame.pivot_x * dest_w;
        let dy = d.y - frame.pivot_y * dest_h;

        draw_texture_ex(
            &self.texture,
            dx,
            dy,
            d.color,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                source: Some(Rect::new(frame.x, frame.y, frame.w, frame.h)),
                flip_x: d.flip_x,
                ..Default::default()
            },
        );
        true
    }
}
