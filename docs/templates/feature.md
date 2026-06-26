# Feature Task

**Owner:** Software Engineer · **Branch:** `feat(<module>)/<summary>` · **Commit:** `feat(<module>): ...`

> Reading this cold? Read `ARCHITECTURE.md` in full — features usually cross the client/server
> seam and the protocol.

## The feature
- **What:** _e.g. spectator mode for a 3rd connection instead of rejecting it_
- **Why / player value:** _one line_
- **Surfaces touched:** _client render? server room? protocol? config?_

## Protocol discipline (if messages change)
- Add/rename in **both** places: `ServerMsg` enum (`client/src/client/net.rs`) and the server
  emitter (`room.js` / `lobby.js` / `config.js`). Field names snake_case on both sides.
- New tunable constant → `config.js` + `configMsg()` + `ServerConfig` (Rust).
- New `GameEvent` → wire it through `server_msg_to_events` and the `run_world` / `run_screens`
  match in `bin/game.rs`.

## Definition of done
- [ ] Feature works end to end in two browser tabs.
- [ ] Protocol changes mirrored on both sides; snake_case consistent.
- [ ] Existing flows unbroken: lobby → start → round → reset → win → game-over → room-ended.
- [ ] `cargo fmt` + `pnpm exec biome check` clean.
- [ ] Heartbeat resync still works.
- [ ] PR per `CONTRIBUTING.md`; protocol changes called out explicitly.
