# Sprite Assets Contract

This document is the contract for producing sprite art that the client can load
and render. It describes **where** files go, the **atlas** format, the **manifest
JSON schema**, and the **native pixel dimensions** expected per object.

The client loads sprites through `client/src/client/render/atlas.rs`.

## File location & naming

Drop both files into `client/public/` (served at the web root, alongside
`font.ttf` — loaded by bare filename):

| File           | Purpose                                  |
| -------------- | ---------------------------------------- |
| `sprites.png`  | The sprite atlas (single PNG, RGBA).     |
| `sprites.json` | The manifest describing named frames.    |

These names are referenced in `game.rs`:
`Atlas::load("sprites.png", "sprites.json")`. If you change filenames, update
that call.

## Atlas image

- Single **PNG**, RGBA (transparency supported and expected around sprites).
- Loaded with `FilterMode::Nearest` so pixel art stays crisp when scaled — author
  at native resolution, do not pre-scale or anti-alias edges.
- Any dimensions; frames are located by explicit source rects in the manifest,
  so the packing/layout is up to you.

## Manifest JSON schema

```jsonc
{
  "frames": {
    "<frame_name>": {
      "x": 0,          // source rect left, in atlas pixels (required)
      "y": 0,          // source rect top, in atlas pixels  (required)
      "w": 32,         // source rect width, in atlas pixels (required)
      "h": 32,         // source rect height, in atlas pixels (required)
      "native_w": 32,  // logical/native sprite width in px  (required)
      "native_h": 32,  // logical/native sprite height in px  (required)
      "pivot_x": 0.0,  // anchor X as 0..1 fraction of frame (optional, default 0.0)
      "pivot_y": 0.0   // anchor Y as 0..1 fraction of frame (optional, default 0.0)
    }
  }
}
```

- **`x,y,w,h`** — the frame's rectangle inside `sprites.png`.
- **`native_w,native_h`** — the sprite's authored logical size. Usually equal to
  `w,h`. Kept separate so a frame can be padded/trimmed in the atlas without
  losing its intended size.
- **`pivot_x,pivot_y`** — the anchor point as a fraction of the frame. `(0,0)` is
  top-left (the default, matching how objects currently position by top-left
  corner). `(0.5,0.5)` is centre. The renderer draws the frame so this pivot
  lands on the requested `(x,y)`.

At render time a frame is scaled so its **width maps to the object's `size`**
(the world's `cfg.size`), and horizontal flip is supported for facing/mirroring.

## Required frames & native dimensions

Every object currently drawn with hardcoded pixels needs a frame. The cowboy has
three states and is tinted per-player (see tinting note below).

| Frame name        | Object / state        | Native size |
| ----------------- | --------------------- | ----------- |
| `cowboy_idle`     | Cowboy, standing/aim  | 32 × 32     |
| `cowboy_falling`  | Cowboy, mid-death     | 32 × 32     |
| `cowboy_fallen`   | Cowboy, dead on ground| 32 × 32     |
| `cow`             | Cow                   | 32 × 32     |
| `bullet`          | Bullet                | 8 × 8       |
| `tumbleweed`      | Tumbleweed            | 32 × 32     |
| `cactus`          | Cactus                | 32 × 32     |
| `tornado`         | Tornado               | 32 × 32     |

The cowboy is drawn in a **32-unit space, lying horizontally** in the current
pixel art (feet at x=0, hat at x=31). New sprite art does not have to match that
exact silhouette, but the frame is scaled to `cfg.size` by width — keep the
cowboy square (32 × 32) so proportions are predictable.

### Player-color tinting (cowboy)

The cowboy is tinted with each player's RGB color. Author the tintable parts of
`cowboy_*` frames in **white/greyscale** so the multiply tint reads correctly;
parts that must stay a fixed color (skin, hat, boots, gun) should be authored at
their final color and will be multiplied by the player color — so if you want a
part unaffected by tint, that is not currently separable in a single frame. If
per-part tint control is needed, raise it and we can split frames or add a mask
channel; the manifest schema is intentionally flexible for future additions.

### Animation (future)

Multi-frame animations are not consumed yet, but the naming scheme supports them:
use `name_frame` keys (e.g. `tumbleweed_0`, `tumbleweed_1`, …) and integration
code can iterate them. No schema change is required to add more frames.

## Placeholder assets

`client/public/sprites.png` + `sprites.json` currently hold a **placeholder**
stub: one flat-colored 32×32 cell per object on a 128×64 atlas, used to prove the
load/render pipeline. (The bullet uses a 32×32 stub cell too; it is scaled down
to its 8px render size, so proportions still read.) Replace them with real art
conforming to the native dimensions in the table above. The bullet already
renders from its atlas frame as a live proof; other objects still use hardcoded
art until sprite integration (a separate issue).
