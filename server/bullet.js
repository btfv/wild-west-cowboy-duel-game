import { GAME_H } from './config.js';

export class Bullet {
  bx;
  y;
  dir;
  spawn_time;
  shooter;
  speed;
  hit_objects = new Set();
  dead = false;

  constructor(bx, by, dir, spawn_time, shooter, speed) {
    this.bx = bx;
    this.y = by;
    this.dir = dir;
    this.spawn_time = spawn_time;
    this.shooter = shooter;
    this.speed = speed;
  }

  posY(t) {
    return this.y + this.dir * this.speed * Math.max(0, t - this.spawn_time);
  }

  outOfBounds(by) {
    return by < 0 || by > GAME_H;
  }
}
