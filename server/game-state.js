import {
  BULLET_SPEED,
  GAME_H,
  GAME_W,
  now,
  OBJ_R,
  OBJ_SPEED,
  SIZE,
  SPEED,
} from './config.js';

const OBJ_SIZE = OBJ_R * 2;
const CACTUS_COUNT = 14;
const ZONE_TOP = GAME_H * 0.15 + SIZE;
const ZONE_BOTTOM = GAME_H * (1 - 0.15) - SIZE;

const PLAYER_SLOTS = [
  { x0: 0, y: 0, x1: GAME_W - SIZE },
  { x0: 0, y: GAME_H - SIZE, x1: GAME_W - SIZE },
];

function rand(min, max) {
  return min + Math.random() * (max - min);
}

function lcg(s) {
  return (Math.imul(s, 2654435761) + 1013904223) >>> 0;
}

function randomPlayerColors() {
  const base = [55, 50, 45].map((b) => b + Math.floor(Math.random() * 200));
  return [base, base.map((c) => 255 - c)];
}

export function makePlayer(r, g, b, slot) {
  const t = now();
  return {
    x0: slot.x0,
    y: slot.y,
    x1: slot.x1,
    speed: SPEED,
    start_time: t,
    move_start: t,
    freeze_end: null,
    frozen_x: null,
    hit: false,
    r,
    g,
    b,
  };
}

export function resetPlayer(player, slot, t) {
  return {
    ...player,
    x0: slot.x0,
    y: slot.y,
    x1: slot.x1,
    start_time: t,
    move_start: t,
    freeze_end: null,
    frozen_x: null,
    hit: false,
  };
}

export function makeBullet(bx, by, dir, spawn_time, shooter) {
  return {
    bx,
    y: by,
    dir,
    speed: BULLET_SPEED,
    spawn_time,
    shooter,
    hit_objects: new Set(),
  };
}

function makeObject(id, x, y, kind, vx) {
  return { id, x, y, kind, vx, hit: false };
}

function spawnObjects() {
  const band = (ZONE_BOTTOM - ZONE_TOP) / 3;
  const y = (i) => rand(ZONE_TOP + i * band, ZONE_TOP + (i + 1) * band);
  const x = () => rand(0, GAME_W - OBJ_SIZE);
  const vx = () => (Math.random() < 0.5 ? OBJ_SPEED : -OBJ_SPEED);
  const bands = [0, 1, 2].sort(() => Math.random() - 0.5);

  return [
    makeObject(0, x(), y(bands[0]), 'cow', 0),
    makeObject(1, x(), y(bands[1]), 'fast', vx()),
    makeObject(2, x(), y(bands[2]), 'slow', vx()),
  ];
}

function generateCactuses() {
  const seed = (Date.now() * Math.random() * 1000) >>> 0;
  const zone_bot = GAME_H * 0.85 - SIZE;
  const bucket_w = GAME_W / CACTUS_COUNT;

  return Array.from({ length: CACTUS_COUNT }, (_, i) => {
    const hx = lcg(seed ^ i);
    const hy = lcg(hx);
    const x = ((hx % 1000) / 1000) * (bucket_w - 14) + i * bucket_w;
    const y = ((hy % 1000) / 1000) * (zone_bot - ZONE_TOP) + ZONE_TOP;

    return [x, y];
  });
}

export function makeGameState(player1Id, player2Id) {
  const [[r1, g1, b1], [r2, g2, b2]] = randomPlayerColors();
  const t = now();

  return {
    players: new Map([
      [player1Id, makePlayer(r1, g1, b1, PLAYER_SLOTS[0])],
      [player2Id, makePlayer(r2, g2, b2, PLAYER_SLOTS[1])],
    ]),
    bullets: [],
    objects: spawnObjects(),
    cactuses: generateCactuses(),
    scores: new Map([
      [player1Id, 0],
      [player2Id, 0],
    ]),
    t,
  };
}

export function resetGameState(state) {
  const t = now();
  const ids = [...state.players.keys()];

  const players = new Map(
    ids.map((id, i) => [
      id,
      resetPlayer(state.players.get(id), PLAYER_SLOTS[i], t),
    ]),
  );

  return {
    ...state,
    players,
    bullets: [],
    objects: spawnObjects(),
    cactuses: generateCactuses(),
    t,
  };
}
