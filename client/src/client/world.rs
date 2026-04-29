use std::collections::HashMap;
use crate::client::config::ServerConfig;
use crate::client::objects::tornado::Tornado;
use crate::client::objects::bullet::Bullet;
use crate::client::objects::cactus::Cactus;
use crate::client::objects::cow::Cow;
use crate::client::objects::cowboy::Cowboy;
use crate::client::objects::map_object::{MapObjectData, ObjKind};
use crate::client::objects::tumbleweed::Tumbleweed;
use crate::client::objects::cowboy::PlayerState;

pub struct World {
    pub my_id: String,
    pub cfg: ServerConfig,
    pub clock_offset: f64,

    pub cowboys: HashMap<String, Cowboy>,
    pub bullets: Vec<Bullet>,
    pub scores: HashMap<String, u32>,
    pub tumbleweedes: Vec<Tumbleweed>,
    pub tornadoes: Vec<Tornado>,
    pub cows: Vec<Cow>,
    pub cactuses: Vec<Cactus>,
}

impl World {
    pub fn new(my_id: String, cfg: ServerConfig, clock_offset: f64) -> Self {
        Self {
            my_id,
            cfg,
            clock_offset,
            cowboys: HashMap::new(),
            bullets: Vec::new(),
            scores: HashMap::new(),
            tumbleweedes: Vec::new(),
            tornadoes: Vec::new(),
            cows: Vec::new(),
            cactuses: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, local_now: f64) {
        let now = local_now + self.clock_offset;
        self.bullets.retain(|b| b.alive(now));
        for sc in &mut self.tumbleweedes { sc.update(dt, self.cfg.obj_r); }
        for fc in &mut self.tornadoes { fc.update(dt, self.cfg.obj_r); }
    }

    pub fn now(&self, local_now: f64) -> f64 {
        local_now + self.clock_offset
    }

    pub fn is_my_player_frozen(&self, now: f64) -> bool {
        self.cowboys.get(&self.my_id).map_or(false, |c| c.is_frozen(now))
    }

    pub async fn on_player_joined(&mut self, state: PlayerState) {
        if let Some(c) = self.cowboys.get_mut(&state.id) {
            c.sync(state);
        } else {
            let id = state.id.clone();
            self.cowboys.insert(id, Cowboy::new(state).await);
        }
    }

    pub fn on_player_left(&mut self, id: String) {
        self.cowboys.remove(&id);
    }

    pub fn on_player_hit(&mut self, x: f32, y: f32, local_now: f64) {
        let now = self.now(local_now);
        let hit = self.cowboys.iter()
            .min_by_key(|(_, c)| {
                let (px, py) = c.state.pos(now);
                ((px - x).powi(2) + (py - y).powi(2)) as i32
            })
            .map(|(pid, c)| {
                let (px, py) = c.state.pos(now);
                (pid.clone(), px, py)
            });
        if let Some((pid, px, py)) = hit {
            if let Some(c) = self.cowboys.get_mut(&pid) {
                c.die(local_now, px, py);
            }
        }
    }

    pub fn on_bullet_fired(&mut self, x: f32, y: f32, dir: f32, spawn_time: f64) {
        self.bullets.push(Bullet { x, y, dir, speed: self.cfg.bullet_speed, spawn_time });
    }

    pub fn on_bullet_modified(&mut self, _obj_id: u32, bx: f32, by: f32, speed: f32, dir: f32, spawn_time: f64) {
        if let Some(b) = self.bullets.iter_mut().find(|b| (b.x - bx).abs() < 2.0) {
            b.y = by; b.speed = speed; b.dir = dir; b.spawn_time = spawn_time;
        }
    }

    pub fn on_bullet_dead(&mut self, x: f32) {
        self.bullets.retain(|b| (b.x - x).abs() > 2.0);
    }

    pub async fn on_objects_spawned(&mut self, objects: Vec<MapObjectData>) {
        let mut new_slow: Vec<Tumbleweed> = Vec::new();
        let mut new_fast: Vec<Tornado> = Vec::new();
        let mut new_cows: Vec<Cow> = Vec::new();
        for MapObjectData { id, x, y, kind, vx } in objects {
            match kind {
                ObjKind::Slow => {
                    let anim = self.tumbleweedes.iter().find(|o| o.id == id).map_or(0.0, |o| o.anim);
                    new_slow.push(Tumbleweed { id, x, y, vx, anim });
                }
                ObjKind::Fast => {
                    let anim = self.tornadoes.iter().find(|o| o.id == id).map_or(0.0, |o| o.anim);
                    new_fast.push(Tornado { id, x, y, vx, anim });
                }
                ObjKind::Cow => {
                    let mut cow = Cow::new(id, x, y).await;
                    if let Some(existing) = self.cows.iter().find(|c| c.id == id) {
                        cow.hit_at = existing.hit_at;
                    }
                    new_cows.push(cow);
                }
            }
        }
        // Keep cows that are mid-death-animation but absent from the server list
        // (server stops broadcasting hit cows immediately).
        for cow in &self.cows {
            if cow.hit_at.is_some() && !new_cows.iter().any(|c| c.id == cow.id) {
                new_cows.push(cow.clone());
            }
        }
        self.tumbleweedes = new_slow;
        self.tornadoes = new_fast;
        self.cows = new_cows;
    }

    pub fn on_cow_hit(&mut self, id: u32, local_now: f64) {
        if let Some(cow) = self.cows.iter_mut().find(|c| c.id == id) {
            cow.hit(local_now);
        }
    }

    pub fn on_cactuses_spawned(&mut self, positions: Vec<(f32, f32)>) {
        self.cactuses = positions.into_iter().map(|(x, y)| Cactus { x, y }).collect();
    }

    pub fn on_scores_updated(&mut self, scores: HashMap<String, u32>) {
        self.scores = scores;
    }

    pub fn on_reset(&mut self) {
        self.bullets.clear();
        self.cows.clear();
        for c in self.cowboys.values_mut() { c.reset(); }
    }
}
