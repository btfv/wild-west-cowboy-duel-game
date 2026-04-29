# Wild West Cowboy Duel

A simple multiplayer 2D browser game for duos. Two cowboys (each controlled by player) shoot at each other across a wild west map.

## Stack

- **Client** — Rust + [macroquad](https://macroquad.rs/), compiled to WASM
- **Server** — Node.js + WebSockets

## How to play

Two players join the same room via URL parameter. Each player controls a cowboy that slides back and forth. Press the action key to fire a bullet. First to 5 hits wins.

Three objects roam the map and interact with bullets:

- **Cow** — blocks the bullet on contact
- **Tornado** — speeds up a bullet
- **Tumbleweed** — slows down a bullet

## Prerequisites

- **Node.js** (v24+) and **pnpm**
- **Rust** toolchain — install via [rustup](https://rustup.rs/):

- **WASM target** for Rust:

```bash
rustup target add wasm32-unknown-unknown
```

## Development

Install dependencies:

```bash
pnpm i
```

Start the WebSocket server:

```bash
PORT=... pnpm run server
```

Start the client dev server:

```bash
WS_URL=... pnpm run dev
```
