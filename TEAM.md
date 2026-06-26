# Team & Hiring Plan

How we staff Wild West Cowboy Duel as work arrives across four streams: **art**,
**game logic**, **features**, and **bug fixes**. This plan is grounded in what the codebase
actually looks like today (see `ARCHITECTURE.md`).

## The constraint that shapes everything

**All "pixel art" today is Rust code.** Every sprite is a stack of `draw_rectangle` calls inside
an object's `draw()` method (`client/src/client/objects/*.rs`). There are no PNGs, no atlas, no
`load_texture`. The only file asset is a font.

That has two consequences:

1. A pixel artist **cannot contribute today** without writing Rust — which defeats the point of
   hiring an artist.
2. So our **first engineering investment** is an asset pipeline that turns image files into
   on-screen sprites. Until that exists, art and code are the same job. After it exists, they
   cleanly separate. This is the seam the whole plan is built around.

---

## Recommended roles

### 1. Software Engineer — logic, features, bugs (hire first)
Owns the Rust client and Node server: the tick loop, collision/scoring, networking, new game
modes, and bug fixes. **Their first project is the asset pipeline** that unblocks the Designer.

*Why this split:* logic and rendering are tightly coupled in Rust today; three of the four work
streams (logic, features, bugs) are pure engineering. One strong generalist engineer covers them.

### 2. Designer — pixel art / sprites (hire second, after the pipeline lands)
Owns the *look*: cowboy/cow/object sprites, animation frames, palettes, backgrounds, UI polish.
Works in image files (Aseprite/PNG), **not** Rust — once the pipeline exists.

*Why a separate role:* visual craft (palette, silhouette, animation timing) is a different skill
from game programming. The procedural-art situation is an accident of there being no pipeline, not
a reason to keep art-as-code forever.

### Later (not yet)
- **Second engineer / backend-leaning** once concurrent matches, persistence, or matchmaking
  beyond room-codes is needed (the server is in-memory and single-process today).
- **QA / playtester** once we ship modes faster than we can manually verify.

---

## The seam: Designer ⇄ Engineer

The boundary must be a **file format and a folder**, so neither side blocks the other.

### What the Designer hands off
- **Format:** PNG with transparency (authored in Aseprite; `.aseprite` source committed
  alongside the export).
- **Location:** `client/public/assets/sprites/<name>.png` (+ source in `assets/src/`).
- **Sizing:** authored on the existing unit grids so collision boxes don't change —
  cowboy **32×32**, map objects (cow/tornado/tumbleweed) **28×28** (`obj_r*2`). Animation frames
  laid out horizontally in a strip; frame size documented in the manifest.
- **Palette:** a shared `palette.png`/`.gpl` in `assets/src/`. New colors get added there first.
- **Manifest:** one `sprites.json` mapping logical name → file, frame size, frame count, fps,
  and anchor point. **This is the contract.** The engineer codes against the manifest, not
  against any specific drawing.

### How the Engineer consumes it
- Loads atlas/PNGs with macroquad `load_texture`, reads `sprites.json`, and replaces each
  procedural `draw()` with a textured `draw_texture_ex` keyed by logical name + frame.
- Per-player tint (cowboy RGB from the server) is applied as a color multiply, so the Designer
  authors **one** grayscale/neutral cowboy and the engine recolors it.
- The unit grids and the server's `size` / `obj_r` are unchanged, so swapping art **never**
  touches collision or server logic.

### Why this never blocks
- The manifest decouples them: the Designer can iterate on a PNG and the Engineer's code keeps
  working as long as name + frame size match the manifest.
- The Engineer can build the loader against placeholder art before the Designer delivers final art.
- Anything not yet pipelined stays procedural; we migrate sprite-by-sprite, not big-bang.

---

## What stays with me (EM)

- **Triage** incoming work into the four streams and assign by role.
- **Task breakdown** — turn CEO requests into the intake templates in `.github/` (`docs/templates/`),
  each with a definition of done a specialist can pick up cold.
- **Review** every PR (correctness, convention, seam discipline — art PRs don't touch logic and
  vice versa).
- **Integration** — own the protocol contract (`ServerMsg` ↔ server emitter), `config.js`
  constants, and merge order so client/server stay in sync.
- **Solo coverage** until hires land: I currently handle all four streams myself.

---

## Hiring order & day-one readiness

**1st — Software Engineer.** Unblocks everything and builds the pipeline the Designer needs.
Day one needs:
- Working dev env (`CONTRIBUTING.md`): Rust + wasm target, Node 24, pnpm, `WS_URL` understood.
- `ARCHITECTURE.md` read; first task = **asset pipeline** (`load_texture` + `sprites.json`),
  migrating one sprite (cow) as the reference implementation.

**2nd — Designer.** Hire once the pipeline is merged and the cow is rendering from a PNG.
Day one needs:
- Aseprite, the `assets/src/` folder with `palette.png` + the cow `.aseprite` as a worked example.
- The unit-grid sizes (32×32 / 28×28) and `sprites.json` schema.
- An art intake ticket (see template) — they should never need to touch Rust.

Net: the first engineer's first deliverable is *literally* the thing that makes the second hire
productive. That's the clean transition the org is designed for.
