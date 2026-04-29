export const now = () => Date.now() / 1000;

export const GAME_W = 256;
export const GAME_H = 410;
export const SIZE = 45;
export const SPEED = 190;
export const FREEZE_DURATION = 2.0;
export const STRIKE_COOLDOWN = 3.5;
export const BULLET_SPEED = 280;
export const WIN_SCORE = 5;
export const OBJ_R = 19.0;
export const OBJ_SPEED = 18.0; // px/s for slow/fast objects
export const BULLET_SPEED_UP = 1.7;
export const BULLET_SLOW_DOWN = 0.3;

export function aabbHit(px, py, x, y, size) {
  return px >= x && px <= x + size && py >= y && py <= y + size;
}

export function configMsg() {
  return {
    type: 'config',
    game_w: GAME_W,
    game_h: GAME_H,
    size: SIZE,
    speed: SPEED,
    freeze_duration: FREEZE_DURATION,
    strike_cooldown: STRIKE_COOLDOWN,
    bullet_speed: BULLET_SPEED,
    win_score: WIN_SCORE,
    obj_r: OBJ_R,
    server_time: Date.now() / 1000,
  };
}
