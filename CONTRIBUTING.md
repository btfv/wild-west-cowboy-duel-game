# Contributing

How we branch, commit, test, and open PRs. Read `ARCHITECTURE.md` first; pick up work from the
templates in `docs/templates/`.

## Setup

See `README.md` for prerequisites. **One thing the README omits:** the client bakes the server
URL in at compile time via `env!("WS_URL")`, so **`cargo build` fails unless `WS_URL` is set**.

```bash
# Server
cd server && pnpm install && PORT=3000 pnpm start

# Client (WS_URL is required for every build, not just dev)
cd client && WS_URL=ws://localhost:3000 pnpm run dev
```

If `cargo build` errors with `environment variable WS_URL not defined at compile time`, you
forgot to export `WS_URL`.

## Branch naming

`<type>/<short-kebab-summary>`, where `<type>` matches the commit type:

```
feat/extra-life-powerup
fix/bullet-passthrough-on-reset
art/cowboy-idle-sprite
refactor/extract-collision
docs/protocol-notes
```

Branch off `main`. **Never commit to `main` directly.** One PR per logical change.

## Commit format

```
type(module): short message
```

- **Types:** `feat`, `fix`, `perf`, `refactor`, `style`, `test`, `docs`, `build`, `ci`,
  `chore`, `revert`.
- **Module:** the area touched — `client`, `server`, `cowboy`, `tick`, `room`, `net`, `protocol`,
  `assets`, etc.
- Multiple: `feat,fix(client,server): ...`. Breaking change: append `!` →
  `feat(protocol)!: rename hit payload`.
- Imperative mood, lowercase, no trailing period.

```
feat(tick): add ricochet off cactuses
fix(room): stop ticking after a player disconnects
art(assets): add tornado idle strip
```

## Testing before a PR

There is no automated test suite yet, so verification is **manual and category-specific**. A PR
must state what you ran and what you observed.

### Art / sprites
- Build the client (`WS_URL=ws://localhost:3000 pnpm run dev`) and open two browser tabs with the
  same `?room=` to see the sprite in context.
- Check both player tints, every animation state (idle / falling / fallen for characters), and
  that the sprite sits inside its collision box (unit grid: 32×32 cowboy, 28×28 objects).
- Confirm no logic files changed — art PRs touch only `objects/*` draw code or `assets/`.

### Game logic
- `cd server && pnpm exec biome check` (formatting/lint).
- Play a full round in two tabs: fire, hit, score, win at 5, reset, object interactions
  (cow blocks / tornado speeds up / tumbleweed slows).
- Anything touching `tick.js` / `game-state.js`: keep them **pure** (no I/O, no `Date.now()`
  outside the passed `t`) and reason through the event list they emit.

### Features
- Both of the above as relevant. If it changes the protocol, update **both** `ServerMsg`
  (`client/src/client/net.rs`) and the server emitter, and call it out in the PR.
- Verify the 3 s heartbeat resync still works (a late-joining/lagging tab should converge).

### Bug fixes
- Reproduce the bug first, capture the steps, then show it gone with the same steps.
- Add a regression note to the PR (and a test once we have a harness).

## Formatting

- **Rust:** `cargo fmt` before committing.
- **JS (server):** `pnpm run format` (Biome) — config in `server/biome.json`.

## Pull requests

Open against `main`. Include:
- **Title** — `type(module): summary`.
- **Summary** — what and why.
- **Changes** — bullet list.
- **Testing** — the category checklist above, with what you actually observed.

The EM reviews every PR for correctness, convention, and **seam discipline** (art PRs don't touch
logic; protocol changes update both sides).
