# Game Logic Task

**Owner:** Software Engineer · **Branch:** `feat|fix(<module>)/<summary>` · **Commit:** `feat|fix|perf(tick|game-state|...): ...`

> Reading this cold? See `ARCHITECTURE.md` → *Game state & the tick loop*. The server is
> authoritative; `tick.js` and `game-state.js` are **pure** and must stay that way.

## The change
- **Rule / mechanic:** _e.g. tornado now also curves the bullet sideways_
- **Affected modules:** _e.g. `server/tick.js`, `server/config.js`_
- **Constants touched:** _e.g. `BULLET_SPEED_UP`_

## Rules of the road
- Keep `tick.js` / `game-state.js` **pure**: input is `(state, dt, t)`, output is
  `{ nextState, events }`. No sockets, no `Date.now()` except the passed `t`.
- If the client must react, emit an **event** and have `Room` broadcast it — don't send from the
  core.
- Tunable numbers live in `config.js`; if the client needs one, add it to `configMsg()` **and**
  `ServerConfig` (`client/src/client/config.rs`).
- Changing a broadcast shape = a protocol change → update both sides (see feature template).

## Definition of done
- [ ] Logic implemented; pure functions stay pure.
- [ ] `pnpm exec biome check` clean (server).
- [ ] Played a full round in two tabs: the new rule behaves; scoring, reset, and win-at-5 still work.
- [ ] Object interactions still correct (cow blocks / tornado speeds up / tumbleweed slows).
- [ ] Heartbeat resync still converges a lagging tab.
- [ ] PR per `CONTRIBUTING.md`, with the observed behavior described.
