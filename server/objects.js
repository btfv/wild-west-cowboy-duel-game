import {
  GAME_W,
  OBJ_R,
  OBJ_SPEED,
  BULLET_SPEED_UP,
  BULLET_SLOW_DOWN,
  aabbHit,
} from "./config.js";

export const OBJ_SIZE = OBJ_R * 2;

class MapObject {
  id;
  x;
  y;
  kind;
  vx;
  hit = false;

  constructor(id, x, y, kind, vx) {
    this.id = id;
    this.x = x;
    this.y = y;
    this.kind = kind;
    this.vx = vx;
  }

  hitTest(bx, by) {
    return aabbHit(bx, by, this.x, this.y, OBJ_SIZE);
  }

  step(dt) {
    if (this.hit || this.vx === 0) return;
    this.x += this.vx * dt;
    if (this.x < 0) {
      this.x = 0;
      this.vx = Math.abs(this.vx);
    }
    if (this.x + OBJ_SIZE > GAME_W) {
      this.x = GAME_W - OBJ_SIZE;
      this.vx = -Math.abs(this.vx);
    }
  }

  toData() {
    return { id: this.id, x: this.x, y: this.y, kind: this.kind, vx: this.vx };
  }

  // Returns broadcast message or null
  modifyBullet(b, by, t) {
    return null;
  }
}

export class CowObject extends MapObject {
  constructor(id, x, y) {
    super(id, x, y, "cow", 0);
  }

  modifyBullet(b, by) {
    this.hit = true;
    b.dead = true;
    return { type: "hit", id: String(this.id), x: b.bx, y: by };
  }
}

export class SlowObject extends MapObject {
  constructor(id, x, y, vx) {
    super(id, x, y, "slow", vx);
  }

  modifyBullet(b, by, t) {
    b.y = by;
    b.spawn_time = t;
    b.speed *= BULLET_SLOW_DOWN;
    return {
      type: "bullet_mod",
      obj_id: String(this.id),
      bx: b.bx,
      by,
      speed: b.speed,
      dir: b.dir,
      spawn_time: t,
    };
  }
}

export class FastObject extends MapObject {
  constructor(id, x, y, vx) {
    super(id, x, y, "fast", vx);
  }

  modifyBullet(b, by, t) {
    b.y = by;
    b.spawn_time = t;
    b.speed *= BULLET_SPEED_UP;

    return {
      type: "bullet_mod",
      obj_id: String(this.id),
      bx: b.bx,
      by,
      speed: b.speed,
      dir: b.dir,
      spawn_time: t,
    };
  }
}

export function randomVx() {
  return Math.random() < 0.5 ? OBJ_SPEED : -OBJ_SPEED;
}
