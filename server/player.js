import {
  aabbHit,
  FREEZE_DURATION,
  GAME_H,
  now,
  SIZE,
  SPEED,
  STRIKE_COOLDOWN,
} from './config.js';

export class Player {
  x0 = 0;
  y = 0;
  x1 = 0;
  speed = SPEED;
  start_time;
  move_start;
  freeze_end = null;
  frozen_x = null;
  hit = false;
  r;
  g;
  b;

  constructor(r, g, b) {
    this.r = r;
    this.g = g;
    this.b = b;
    this.start_time = now();
    this.move_start = now();
  }

  posAt(t) {
    if (this.freeze_end && t < this.freeze_end)
      return { x: this.frozen_x, y: this.y };

    const elapsed = Math.max(0, t - this.start_time);
    const dist = Math.abs(this.x1 - this.x0);
    if (dist < 0.001) return { x: this.x0, y: this.y };

    const traveled = elapsed * this.speed;
    const cycle = (traveled / dist) % 2;
    const progress = cycle <= 1 ? cycle : 2 - cycle;

    return { x: this.x0 + (this.x1 - this.x0) * progress, y: this.y };
  }

  hitTest(bx, by, t) {
    const { x, y } = this.posAt(t);
    return aabbHit(bx, by, x, y, SIZE);
  }

  resetToSlot(slot, t) {
    this.x0 = slot.x0;
    this.y = slot.y;
    this.x1 = slot.x1;
    this.start_time = t;
    this.move_start = t;
    this.freeze_end = null;
    this.frozen_x = null;
    this.hit = false;
  }

  // Returns bullet spawn data, or null if freeze not allowed
  applyFreeze(t) {
    if (this.freeze_end && t < this.freeze_end) return null;
    if (t - this.move_start < STRIKE_COOLDOWN) return null;

    const dist = Math.abs(this.x1 - this.x0);
    const elapsed = Math.max(0, t - this.start_time);
    const ft = ((elapsed * this.speed) / dist) % 2;
    const progress = ft <= 1 ? ft : 2 - ft;
    const frozen_x = this.x0 + (this.x1 - this.x0) * progress;
    const freeze_end = t + FREEZE_DURATION;
    // set start_time so posAt() resumes from frozen_x after freeze ends
    this.frozen_x = frozen_x;
    this.start_time = freeze_end - (progress * dist) / this.speed;
    this.freeze_end = freeze_end;
    this.move_start = t + FREEZE_DURATION; // cooldown starts when movement resumes

    return {
      bx: frozen_x + SIZE / 2,
      by: this.y + SIZE / 2,
      dir: this.y < GAME_H / 2 ? 1 : -1,
    };
  }

  toState() {
    return {
      x0: this.x0,
      y: this.y,
      x1: this.x1,
      speed: this.speed,
      start_time: this.start_time,
      move_start: this.move_start,
      freeze_end: this.freeze_end,
      frozen_x: this.frozen_x,
      hit: this.hit,
      r: this.r,
      g: this.g,
      b: this.b,
    };
  }
}
