import {
  aabbHit,
  BULLET_SLOW_DOWN,
  BULLET_SPEED_UP,
  FREEZE_DURATION,
  GAME_H,
  GAME_W,
  OBJ_R,
  SIZE,
  STRIKE_COOLDOWN,
  WIN_SCORE,
} from './config.js';
import { makeBullet, resetPlayer } from './game-state.js';

const OBJ_SIZE = OBJ_R * 2;

// --- pure physics helpers ---

function playerPosAt(player, t) {
  if (player.freeze_end && t < player.freeze_end)
    return { x: player.frozen_x, y: player.y };

  const elapsed = Math.max(0, t - player.start_time);
  const dist = Math.abs(player.x1 - player.x0);
  if (dist < 0.001) return { x: player.x0, y: player.y };

  const traveled = elapsed * player.speed;
  const cycle = (traveled / dist) % 2;
  const progress = cycle <= 1 ? cycle : 2 - cycle;

  return { x: player.x0 + (player.x1 - player.x0) * progress, y: player.y };
}

function playerHitTest(player, bx, by, t) {
  const { x, y } = playerPosAt(player, t);
  return aabbHit(bx, by, x, y, SIZE);
}

function bulletPosY(bullet, t) {
  return (
    bullet.y + bullet.dir * bullet.speed * Math.max(0, t - bullet.spawn_time)
  );
}

function stepObject(obj, dt) {
  if (obj.hit || obj.vx === 0) return obj;

  let { x, vx } = obj;
  x += vx * dt;
  if (x < 0) {
    x = 0;
    vx = Math.abs(vx);
  }
  if (x + OBJ_SIZE > GAME_W) {
    x = GAME_W - OBJ_SIZE;
    vx = -Math.abs(vx);
  }

  return { ...obj, x, vx };
}

function objHitTest(obj, bx, by) {
  return aabbHit(bx, by, obj.x, obj.y, OBJ_SIZE);
}

// Returns { bullet, obj, event } — bullet and obj are updated copies
function applyBulletObjCollision(bullet, obj, by, t) {
  const hit_objects = new Set(bullet.hit_objects);
  hit_objects.add(obj.id);

  switch (obj.kind) {
    case 'cow': {
      return {
        bullet: { ...bullet, hit_objects, dead: true },
        obj: { ...obj, hit: true },
        event: { type: 'hit', id: String(obj.id), x: bullet.bx, y: by },
      };
    }
    case 'slow': {
      const speed = bullet.speed * BULLET_SLOW_DOWN;
      return {
        bullet: { ...bullet, hit_objects, y: by, spawn_time: t, speed },
        obj,
        event: {
          type: 'bullet_mod',
          obj_id: String(obj.id),
          bx: bullet.bx,
          by,
          speed,
          dir: bullet.dir,
          spawn_time: t,
        },
      };
    }
    case 'fast': {
      const speed = bullet.speed * BULLET_SPEED_UP;
      return {
        bullet: { ...bullet, hit_objects, y: by, spawn_time: t, speed },
        obj,
        event: {
          type: 'bullet_mod',
          obj_id: String(obj.id),
          bx: bullet.bx,
          by,
          speed,
          dir: bullet.dir,
          spawn_time: t,
        },
      };
    }
  }
}

// --- freeze action (called from Room on player input) ---

export function applyFreeze(player, t) {
  if (player.freeze_end && t < player.freeze_end) return null;
  if (t - player.move_start < STRIKE_COOLDOWN) return null;

  const dist = Math.abs(player.x1 - player.x0);
  const elapsed = Math.max(0, t - player.start_time);
  const ft = ((elapsed * player.speed) / dist) % 2;
  const progress = ft <= 1 ? ft : 2 - ft;
  const frozen_x = player.x0 + (player.x1 - player.x0) * progress;
  const freeze_end = t + FREEZE_DURATION;

  const nextPlayer = {
    ...player,
    frozen_x,
    freeze_end,
    start_time: freeze_end - (progress * dist) / player.speed,
    move_start: t + FREEZE_DURATION,
  };

  const bullet = makeBullet(
    frozen_x + SIZE / 2,
    player.y + SIZE / 2,
    player.y < GAME_H / 2 ? 1 : -1,
    t,
    null, // shooter id set by caller
  );

  return { nextPlayer, bullet };
}

// --- main tick ---

// Pure: (GameState, dt, t) -> { nextState, events[] }
export function tick(state, dt, t) {
  const events = [];

  // Step objects
  const objects = state.objects.map((obj) => stepObject(obj, dt));

  // Step bullets + resolve collisions
  const players = new Map(state.players);
  const scores = new Map(state.scores);
  const updatedObjects = [...objects];

  const bullets = state.bullets
    .map((bullet) => {
      const by = bulletPosY(bullet, t);

      if (by < 0 || by > GAME_H) return { ...bullet, dead: true };

      let current = bullet;

      // Bullet vs objects
      for (let i = 0; i < updatedObjects.length; i++) {
        const obj = updatedObjects[i];
        if (current.dead || current.hit_objects.has(obj.id) || obj.hit)
          continue;
        if (!objHitTest(obj, current.bx, by)) continue;

        const result = applyBulletObjCollision(current, obj, by, t);
        current = result.bullet;
        updatedObjects[i] = result.obj;
        events.push(result.event);
      }

      // Bullet vs players
      for (const [id, player] of players) {
        if (current.dead || id === current.shooter || player.hit) continue;
        if (!playerHitTest(player, current.bx, by, t)) continue;

        current = { ...current, dead: true };
        players.set(id, { ...player, hit: true });

        const score = (scores.get(current.shooter) ?? 0) + 1;
        scores.set(current.shooter, score);

        events.push({ type: 'hit', id, x: current.bx, y: by });
        events.push({ type: 'scores', scores: Object.fromEntries(scores) });

        if (score >= WIN_SCORE) {
          events.push({
            type: 'game_over',
            winner: current.shooter,
            scores: Object.fromEntries(scores),
          });
        } else {
          events.push({ type: 'schedule_reset' });
        }
      }

      return current;
    })
    .filter((b) => !b.dead);

  const nextState = {
    ...state,
    players,
    bullets,
    objects: updatedObjects,
    scores,
  };

  return { nextState, events };
}
