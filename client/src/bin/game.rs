use macroquad::prelude::*;

#[path = "../client/mod.rs"]
mod client;

use client::config::ServerConfig;
use client::constants::*;
use client::event_bus::EventBus;
use client::events::GameEvent;
use client::net::*;
use client::render::*;
use client::screens::connecting::ConnectingScreen;
use client::screens::game_over::GameOverScreen;
use client::screens::room_ended::RoomEndedScreen;
use client::screens::server_full::ServerFullScreen;
use client::screens::waiting::WaitingScreen;
use client::screens::win_overlay::WinOverlay;
use client::screens::{Screen, ScreenContext};
use client::utils::*;
use client::world::World;

fn conf() -> Conf {
    Conf {
        window_title: "Multiplayer".to_string(),
        window_resizable: true,
        ..Default::default()
    }
}

fn server_msg_to_events(text: &str, bus: &mut EventBus<GameEvent>) {
    let Ok(msg) = serde_json::from_str::<ServerMsg>(text) else {
        return;
    };
    match msg {
        ServerMsg::State(p) => bus.emit(GameEvent::PlayerJoined(p)),
        ServerMsg::Leave { id } => bus.emit(GameEvent::PlayerLeft(id)),
        ServerMsg::Hit { id, x, y } => bus.emit(GameEvent::Hit { id, x, y }),
        ServerMsg::Bullet {
            x,
            y,
            dir,
            spawn_time,
        } => bus.emit(GameEvent::BulletFired {
            x,
            y,
            dir,
            spawn_time,
        }),
        ServerMsg::BulletMod {
            obj_id,
            bx,
            by,
            speed,
            dir,
            spawn_time,
        } => bus.emit(GameEvent::BulletModified {
            obj_id,
            bx,
            by,
            speed,
            dir,
            spawn_time,
        }),
        ServerMsg::Objects { objects } => bus.emit(GameEvent::ObjectsSpawned(objects)),
        ServerMsg::Cactuses { positions } => bus.emit(GameEvent::CactusesSpawned(positions)),
        ServerMsg::Scores { scores } => bus.emit(GameEvent::ScoresUpdated(scores)),
        ServerMsg::Win { id } => bus.emit(GameEvent::Win(id)),
        ServerMsg::GameOver { winner, scores } => bus.emit(GameEvent::ShowGameOver {
            winner_id: winner,
            scores,
        }),
        ServerMsg::Reset => bus.emit(GameEvent::Reset),
        ServerMsg::Waiting { room_id } => bus.emit(GameEvent::ShowWaiting(room_id)),
        ServerMsg::Full => bus.emit(GameEvent::ShowServerFull),
        ServerMsg::RoomEnded => bus.emit(GameEvent::ShowRoomEnded),
        ServerMsg::Start => bus.emit(GameEvent::GameStarted),
        ServerMsg::Config(_) => {}
    }
}

fn game_camera(rctx: &RenderCtx) {
    let sw = screen_width();
    let sh = screen_height();
    let scale = (sw / GAME_W).min(sh / GAME_H);
    let ox = (sw - GAME_W * scale) / 2.0;
    let oy = (sh - GAME_H * scale) / 2.0;
    set_game_camera(ox, oy, sw, sh, scale);
    draw_background(rctx.bg_seed);
}

fn make_screen_ctx(rctx: &RenderCtx) -> ScreenContext<'_> {
    let sw = screen_width();
    let sh = screen_height();
    let scale = (sw / GAME_W).min(sh / GAME_H);
    let ox = (sw - GAME_W * scale) / 2.0;
    let oy = (sh - GAME_H * scale) / 2.0;
    ScreenContext {
        font: &rctx.font,
        sw,
        sh,
        ox,
        oy,
        scale,
        share_label: js_share_action_label(),
    }
}

async fn run_screens(
    id: &str,
    rctx: &mut RenderCtx,
    bus: &mut EventBus<GameEvent>,
    current_screen: &mut Box<dyn Screen>,
) {
    let mut room_url: Option<String> = None;

    loop {
        let page_origin = js_page_origin();

        while let Some(text) = js_ws_try_recv() {
            server_msg_to_events(&text, bus);
        }

        let mut game_started = false;
        let mut deferred = Vec::new();
        for event in bus.drain().collect::<Vec<_>>() {
            match event {
                GameEvent::GameStarted => {
                    game_started = true;
                }
                GameEvent::ShowWaiting(room_id) => {
                    let url = join_url(&page_origin, &room_id);
                    *current_screen = Box::new(WaitingScreen::new(&url));
                    room_url = Some(url);
                }
                GameEvent::ShowServerFull => {
                    *current_screen = Box::new(ServerFullScreen);
                }
                GameEvent::ShowRoomEnded => {
                    *current_screen = Box::new(RoomEndedScreen {});
                }
                GameEvent::ShowGameOver { winner_id, scores } => {
                    *current_screen = Box::new(GameOverScreen {
                        winner_id,
                        scores,
                        my_id: id.to_string(),
                    });
                }
                e => deferred.push(e),
            }
        }
        for e in deferred {
            bus.emit(e);
        }
        if game_started {
            return;
        }

        let ctx = make_screen_ctx(rctx);

        clear_background(BLACK);
        game_camera(rctx);

        for event in current_screen.draw(&ctx) {
            match event {
                GameEvent::OpenBaseUrl => js_open_url(&page_origin),
                GameEvent::CopyRoomLink => {
                    let url = room_url.as_deref().unwrap_or(&page_origin);
                    js_share_action(url);
                }
                _ => {}
            }
        }

        set_default_camera();
        next_frame().await;
    }
}

async fn run_world(
    id: &str,
    cfg: &ServerConfig,
    clock_offset: f64,
    rctx: &RenderCtx,
    bus: &mut EventBus<GameEvent>,
) {
    let mut world = World::new(id.to_string(), cfg.clone(), clock_offset);
    let mut win_overlay: Option<WinOverlay> = None;

    loop {
        let local_now = macroquad::miniquad::date::now();
        let now = world.now(local_now);

        if !world.is_my_player_frozen(now)
            && (is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space))
        {
            bus.emit(GameEvent::FreezeRequested);
        }

        while let Some(text) = js_ws_try_recv() {
            server_msg_to_events(&text, bus);
        }

        for event in bus.drain().collect::<Vec<_>>() {
            match event {
                GameEvent::PlayerJoined(p) => {
                    world.on_player_joined(p).await;
                }
                GameEvent::PlayerLeft(pid) => {
                    world.on_player_left(pid);
                }
                GameEvent::Hit { id, x, y } => {
                    world.on_bullet_dead(x);
                    if let Ok(obj_id) = id.parse::<u32>() {
                        world.on_cow_hit(obj_id, local_now);
                    } else {
                        world.on_player_hit(x, y, local_now);
                    }
                }
                GameEvent::BulletFired {
                    x,
                    y,
                    dir,
                    spawn_time,
                } => {
                    world.on_bullet_fired(x, y, dir, spawn_time);
                }
                GameEvent::BulletModified {
                    obj_id,
                    bx,
                    by,
                    speed,
                    dir,
                    spawn_time,
                } => {
                    world.on_bullet_modified(
                        obj_id.parse().unwrap_or(0),
                        bx,
                        by,
                        speed,
                        dir,
                        spawn_time,
                    );
                }
                GameEvent::ObjectsSpawned(objects) => {
                    world.on_objects_spawned(objects).await;
                }

                GameEvent::CactusesSpawned(positions) => {
                    world.on_cactuses_spawned(positions);
                }
                GameEvent::ScoresUpdated(scores) => {
                    world.on_scores_updated(scores);
                }
                GameEvent::Reset => {
                    world.on_reset();
                }
                GameEvent::Win(winner_id) => {
                    let label = if winner_id == id {
                        "You win!"
                    } else {
                        "Opp wins!"
                    };
                    win_overlay = Some(WinOverlay::new(label.to_string(), local_now));
                }
                GameEvent::ShowGameOver { winner_id, scores } => {
                    bus.emit(GameEvent::ShowGameOver { winner_id, scores });
                    return;
                }
                GameEvent::FreezeRequested => {
                    js_ws_send(r#"{"type":"freeze"}"#);
                }
                _ => {}
            }
        }

        let ctx = make_screen_ctx(rctx);

        world.update(get_frame_time(), local_now);

        clear_background(BLACK);
        game_camera(rctx);
        world.draw(&rctx.font, local_now);

        if let Some(ref overlay) = win_overlay {
            if overlay.expired(local_now) {
                win_overlay = None;
            } else {
                overlay.draw(&ctx);
            }
        }

        set_default_camera();
        next_frame().await;
    }
}

#[macroquad::main(conf)]
async fn main() {
    let id = make_id();

    let room_id = {
        let from_url = js_get_query_param("room");
        if from_url.is_empty() {
            make_room_id()
        } else {
            from_url
        }
    };

    js_ws_connect(&js_ws_url(&room_id, &id));

    let font = load_ttf_font("font.ttf").await.unwrap();

    loop {
        if js_ws_connected() {
            break;
        }
        if js_ws_failed() {
            loop {
                clear_background(BLACK);
                draw_centered("Connection failed", screen_height() / 2.0, 16.0, RED, &font);
                next_frame().await;
            }
        }
        next_frame().await;
    }

    js_ws_send(&format!(r#"{{"type":"join"}}"#));

    let cfg: ServerConfig = loop {
        if let Some(text) = js_ws_try_recv() {
            if let Ok(ServerMsg::Config(c)) = serde_json::from_str::<ServerMsg>(&text) {
                break c;
            }
        }
        clear_background(BLACK);
        draw_centered("Connecting...", screen_height() / 2.0, 12.0, WHITE, &font);
        next_frame().await;
    };
    let clock_offset = cfg.server_time - macroquad::miniquad::date::now();

    let mut rctx = RenderCtx {
        font,
        bg_seed: (macroquad::miniquad::date::now() * 1_000_000.0) as u32,
    };

    let mut bus: EventBus<GameEvent> = EventBus::new();
    let mut current_screen: Box<dyn Screen> = Box::new(ConnectingScreen);

    loop {
        run_screens(&id, &mut rctx, &mut bus, &mut current_screen).await;
        run_world(&id, &cfg, clock_offset, &rctx, &mut bus).await;
        current_screen = Box::new(ConnectingScreen);
    }
}
