# Architecture

Wild West Cowboy Duel is a two-player browser game. The **server is authoritative**
over all game logic; the **client is a thin renderer** that predicts motion from
deterministic formulas and reconciles against periodic server state.

```
Browser (Rust → WASM, macroquad)  ⇄  WebSocket (JSON)  ⇄  Node.js server (ws)
        client/                                                 server/
```

- **Client** — Rust + [macroquad](https://macroquad.rs/) compiled to `wasm32-unknown-unknown`,
  loaded by a thin JS shim (`mq_js_bundle.js` + `quad-net.js`) in `public/index.html`.
- **Server** — Node.js (ESM) + the `ws` WebSocket library. No database; all state is in memory.

---

## Server (`/server`)

Plain Node, no framework. Entry point `server.js`. Pure-function game core (`tick.js`,
`game-state.js`) kept separate from the stateful transport/room classes.

| File | Responsibility |
|------|----------------|
| `server.js` | WebSocket server. Validates `?room=&id=` query params, wraps `socket.send` to auto-`JSON.stringify`, hands the socket to the registry. |
| `registry.js` | `Map<roomId, RoomEntry>`. First player into a room creates a `Lobby`; second player upgrades it to a `Room`. Deletes the room on `ended`. |
| `lobby.js` | `RoomEntry` base class + `Lobby` (one waiting player). `connect()` upgrades lobby→room via the `onUpgrade` callback. |
| `room.js` | The live match: holds two sockets, runs the tick loop, broadcasts state, handles `freeze` input, disconnects, resets, and game-over. |
| `tick.js` | **Pure** physics: `tick(state, dt, t) → { nextState, events }`. Bullet motion, object collisions, player hit detection, scoring. Plus `applyFreeze` (fire action). |
| `game-state.js` | **Pure** state constructors: players, bullets, map objects, cactuses, scores. `makeGameState` / `resetGameState`. |
| `config.js` | Shared numeric constants (board size, speeds, cooldowns, win score), `aabbHit`, and `configMsg()` — the config payload sent to clients. |

### Rooms & matchmaking

Matchmaking is **room-code based, not a queue**. Two players share a URL with the same
`?room=<code>`. Flow (`registry.js`):

1. Player A connects → no entry exists → `Lobby` created, A receives `config` then `waiting`.
2. Player B connects with the same `room` → `lobby.connect()` fires `onUpgrade` → a `Room` is
   built from both players and replaces the lobby in the registry.
3. A **third** connection to a live room gets `{ type: 'full' }` and is effectively rejected
   (the room ignores it).

Room teardown: either socket closing, or a `game_over`, calls `Room#end()` → clears intervals,
emits `ended` → registry deletes the room.

### Game state & the tick loop

`Room` runs two intervals:

- **Tick** every `TICK_MS = 50` (20 Hz): calls the pure `tick()`, swaps in `nextState`,
  dispatches the returned `events` to sockets.
- **Heartbeat** every `3000 ms`: re-broadcasts full player + object + score state so a client
  that missed an event resynchronizes.

`tick()` is pure and returns events rather than sending them — the `Room` owns all I/O. The
game state shape:

```
{
  players:  Map<id, { x0, y, x1, speed, start_time, move_start,
                      freeze_end, frozen_x, hit, r, g, b }>,
  bullets:  [{ bx, y, dir, speed, spawn_time, shooter, hit_objects:Set }],
  objects:  [{ id, x, y, kind:'cow'|'fast'|'slow', vx, hit }],
  cactuses: [[x, y], ...],   // decorative only, never collide
  scores:   Map<id, number>,
}
```

**Motion is formula-based, not integrated per frame.** A player's position at time `t` is
derived from `start_time`, `speed`, and the `[x0, x1]` track (a ping-pong slide). Bullets are
`y = spawn_y + dir * speed * (t - spawn_time)`. This is what lets the client reproduce motion
locally from a single state message (see *Clock sync* below).

The three roaming `objects` interact with bullets in `applyBulletObjCollision`:

- `cow` — blocks the bullet (bullet dies, cow falls).
- `fast` (Tornado) — speeds the bullet up (`× BULLET_SPEED_UP`).
- `slow` (Tumbleweed) — slows the bullet down (`× BULLET_SLOW_DOWN`).

Each bullet remembers which object ids it has already hit (`hit_objects`) so it modifies once.

---

## Client (`/client`)

Rust crate `game-client`, binary `game` (`src/bin/game.rs`). All game code lives under
`src/client/`. `src/lib.rs` is intentionally empty.

### Game loop

`main()` (`bin/game.rs`) does: make ids → connect WS → load font → wait for `config` →
compute clock offset → then loop forever over two phases:

1. **`run_screens`** — menu/overlay states (connecting, waiting, server-full, room-ended,
   game-over). Returns when a `start` is received.
2. **`run_world`** — the live match render loop. Returns on game-over, dropping back to screens.

Each phase is its own async loop running at `macroquad`'s frame rate (`next_frame().await`).
Every frame: drain inbound WS messages → translate to `GameEvent`s on the bus → apply events
to `World` → read input → `world.update()` → `world.draw()`.

### Rendering

- **Virtual resolution** is fixed at `GAME_W × GAME_H = 256 × 410` (`constants.rs`). Every frame
  computes a uniform `scale` and letterbox offset (`ox, oy`) to fit the window, set via
  `set_game_camera`. All game coordinates are in this 256×410 space.
- **`World::draw`** (`renderer.rs`) paints in z-order: cactuses → tumbleweeds → tornadoes →
  cows → cowboys → cooldown bar → bullets → score labels.
- **`DrawContext`** carries the font, clock, and server config (`obj_r`, `size`, `strike_cooldown`)
  into each object's `draw`.
- Text uses one TTF (`public/font.ttf`) rasterized at a `128px` atlas and scaled down for
  crispness (`render.rs`).

### Input

Input is read in `run_world`: **left-click or Space** emits `GameEvent::FreezeRequested`, which
sends `{"type":"freeze"}` to the server — the only input message the game sends. There is no
continuous movement input; cowboys auto-slide, and "freeze" stops the slide and fires a bullet.
Input is ignored while the local player is frozen.

### Networking

WASM has no sockets, so networking is delegated to JS via `extern "C"` FFI declared in
`net.rs` and implemented in `public/quad-net.js`:

- `ws_connect / ws_send / ws_recv_len / ws_recv_into` — the socket. Inbound messages are queued
  in JS and pulled with `js_ws_try_recv()` each frame.
- `get_query_param`, `get_page_origin`, `open_url`, `share_action[_label]` — browser glue for
  room links and share/copy buttons.

The server URL is **baked in at compile time** via `env!("WS_URL")` (`net.rs`):

```
js_ws_url() → format!("{WS_URL}?room={room}&id={id}")
```

`WS_URL` must therefore be set when you run `cargo build` — see `CONTRIBUTING.md`.

#### Clock sync

The server sends `server_time` in the `config` message. The client computes
`clock_offset = server_time − local_now` once and applies it everywhere (`World::now`), so the
client can evaluate the same position formulas the server uses against a shared timeline. This
is why a single `state` message is enough to render smooth motion between updates.

---

## Sprites & pixel assets

**There are no image assets.** Every sprite — cowboys, cow, tornado, tumbleweed, cactus,
bullet, background — is drawn **procedurally in Rust** as stacks of `draw_rectangle` calls inside
each object's `draw()` method:

| Sprite | File | Notes |
|--------|------|-------|
| Cowboy (idle / falling / fallen) | `objects/cowboy.rs` | 32×32 unit grid, scaled by `size/32`. Body color is per-player RGB from the server; x auto-mirrors for the gun arm. |
| Cow (idle / falling / fallen) | `objects/cow.rs` | 28-unit grid (`obj_r/14`). |
| Tornado (`fast`) | `objects/tornado.rs` | Banded silhouette + animated debris. |
| Tumbleweed (`slow`) | `objects/tumbleweed.rs` | Rotating dot-ring, animated. |
| Cactus | `objects/cactus.rs` | Static decoration. |
| Bullet | `objects/bullet.rs` | Three rectangles. |
| Background / dunes | `render.rs` `draw_background` | Seeded procedural. |

The only file-based asset is `public/font.ttf` (loaded with `load_ttf_font`). The grids are in
fixed unit space and scaled by the server-provided `size` / `obj_r` at draw time, so art and the
server's collision boxes stay in lockstep.

> **Implication for hiring:** "pixel art" today means editing Rust draw code, not dropping in
> PNGs. Onboarding a Designer cleanly will require a real asset pipeline (texture atlas +
> `load_texture`). This is the central reason for the role split — see `TEAM.md`.

---

## Client ⇄ Server message protocol

JSON over WebSocket. Every message is `{ "type": ..., ... }`. Rust side: `ServerMsg` enum in
`net.rs` (`#[serde(tag = "type", rename_all = "snake_case")]`).

### Client → Server

| `type` | When | Payload |
|--------|------|---------|
| `join` | After WS connects | _(none)_ |
| `freeze` | Player clicks / presses Space | _(none)_ — server computes freeze + fires the bullet |

### Server → Client

| `type` | Meaning | Key fields |
|--------|---------|------------|
| `config` | Sent on connect; constants + `server_time` | `size, speed, strike_cooldown, bullet_speed, win_score, obj_r, server_time` |
| `waiting` | In lobby, awaiting opponent | `room_id` |
| `full` | Room already has two players | — |
| `start` | Opponent joined, match begins | — |
| `state` | One player's full state | `id, x0, y, x1, speed, start_time, move_start, freeze_end, frozen_x, r, g, b` |
| `leave` | A player left | `id` |
| `bullet` | A bullet was fired | `x, y, dir, spawn_time` |
| `bullet_mod` | Bullet sped up / slowed by an object | `obj_id, bx, by, speed, dir, spawn_time` |
| `hit` | Bullet hit a player (numeric-string `id` = object/cow) | `id, x, y` |
| `objects` | Current roaming objects | `objects: [{ id, x, y, kind, vx }]` |
| `cactuses` | Decorative cactus positions | `positions: [[x, y], ...]` |
| `scores` | Score table | `scores: { id: n }` |
| `reset` | Round reset after a point | — |
| `game_over` | Match ended | `winner, scores` |
| `room_ended` | Room torn down | — |

Notes:
- `hit` is overloaded: an `id` that parses as a number is an object/cow hit; otherwise it's a
  player hit (`bin/game.rs` branches on `id.parse::<u32>()`).
- The server pushes; the client never polls. The 3 s heartbeat re-sends `state` + `objects` +
  `scores` for resync.
- Adding a message means editing **both** the `ServerMsg` enum (`net.rs`) and the server emitter —
  keep field names in snake_case on both sides.
