# Art / Sprite Task

**Owner:** Designer · **Branch:** `art/<summary>` · **Commit:** `art(assets|<sprite>): ...`

> Reading this cold? See `ARCHITECTURE.md` → *Sprites & pixel assets* and `TEAM.md` → *The seam*.
> You author image files against a fixed unit grid and a manifest; you do **not** edit Rust logic.

## What to make
- **Sprite / asset:** _e.g. cowboy idle_
- **States / frames:** _e.g. idle, falling, fallen — or a 6-frame walk strip_
- **Reference / vibe:** _link or description_

## Specs (don't deviate without flagging the EM)
- **Grid / size:** cowboy `32×32`, map objects (cow / tornado / tumbleweed) `28×28`. Frames laid
  out horizontally; record frame size + count in the manifest.
- **Format:** PNG with transparency. Commit the `.aseprite` source in `assets/src/`.
- **Palette:** use `assets/src/palette.png`; add new colors there first.
- **Tint:** cowboys are recolored per-player by the engine — author **neutral/grayscale**, not a
  fixed color.
- **Manifest:** add/update the entry in `client/public/assets/sprites/sprites.json`
  (name, file, frame size, frame count, fps, anchor).

## Definition of done
- [ ] PNG export in `client/public/assets/sprites/` + `.aseprite` source in `assets/src/`.
- [ ] `sprites.json` updated and valid.
- [ ] Sprite fits its collision box on the unit grid (no overflow past 32×32 / 28×28).
- [ ] Verified in-game in two browser tabs (same `?room=`): correct in every state, both tints.
- [ ] No Rust **logic** files changed.
- [ ] PR follows `CONTRIBUTING.md` (Title / Summary / Changes / Testing).
