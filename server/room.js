import { now } from './config.js';
import { makeGameState, resetGameState } from './game-state.js';
import { RoomEntry } from './lobby.js';
import { applyFreeze, tick } from './tick.js';

const TICK_MS = 50;
const HEARTBEAT_INTERVAL = 3000;
const RESET_DELAY_MS = 3000;

export class Room extends RoomEntry {
  #state;
  #sockets; // Map<id, socket>
  #intervals = [];

  constructor(player1, player2) {
    super();
    this.#sockets = new Map([
      [player1.id, player1.socket],
      [player2.id, player2.socket],
    ]);
    this.#state = makeGameState(player1.id, player2.id);

    this.#broadcast({ type: 'start' });
    this.#broadcastFullState();
    this.#broadcastPlayerStates();

    this.#intervals.push(
      setInterval(() => this.#tick(), TICK_MS),
      setInterval(() => this.#heartbeat(), HEARTBEAT_INTERVAL),
    );

    for (const [id, socket] of this.#sockets) {
      socket.on('message', (raw) => {
        const parsed = JSON.parse(raw.toString());
        if (parsed.type === 'freeze') this.#onFreeze(id);
      });
      socket.on('close', () => this.#onDisconnect(id));
    }
  }

  connect(_id, socket) {
    socket.send({ type: 'full' });
    return this;
  }

  // --- private ---

  #broadcast(msg) {
    for (const socket of this.#sockets.values()) {
      if (socket.readyState === 1) socket.send(msg);
    }
  }

  #broadcastPlayerStates() {
    for (const [id, player] of this.#state.players) {
      this.#broadcast({ type: 'state', id, ...player });
    }
  }

  #broadcastFullState() {
    this.#broadcast({
      type: 'scores',
      scores: Object.fromEntries(this.#state.scores),
    });
    this.#broadcast({
      type: 'objects',
      objects: this.#state.objects.filter((o) => !o.hit),
    });
    this.#broadcast({ type: 'cactuses', positions: this.#state.cactuses });
  }

  #tick() {
    const t = now();
    const dt = TICK_MS / 1000;
    const { nextState, events } = tick(this.#state, dt, t);
    this.#state = nextState;
    this.#dispatchEvents(events);
  }

  #heartbeat() {
    this.#broadcastPlayerStates();
    this.#broadcastFullState();
  }

  #dispatchEvents(events) {
    for (const event of events) {
      switch (event.type) {
        case 'hit':
        case 'bullet_mod':
        case 'scores':
          this.#broadcast(event);
          break;
        case 'game_over':
          this.#broadcast(event);
          this.#end();
          break;
        case 'schedule_reset':
          setTimeout(() => this.#reset(), RESET_DELAY_MS);
          break;
      }
    }
  }

  #reset() {
    this.#state = resetGameState(this.#state);
    this.#broadcastPlayerStates();
    this.#broadcastFullState();
    this.#broadcast({ type: 'reset' });
  }

  #end() {
    this.#intervals.forEach(clearInterval);
    this.#intervals = [];
    this.emit('ended');
  }

  #onFreeze(id) {
    const player = this.#state.players.get(id);
    if (!player) return;

    const t = now();
    const result = applyFreeze(player, t);
    if (!result) return;

    const { nextPlayer, bullet } = result;
    const nextBullet = { ...bullet, shooter: id };

    const players = new Map(this.#state.players);
    players.set(id, nextPlayer);
    this.#state = {
      ...this.#state,
      players,
      bullets: [...this.#state.bullets, nextBullet],
    };

    this.#broadcast({ type: 'state', id, ...nextPlayer });
    this.#broadcast({
      type: 'bullet',
      x: nextBullet.bx,
      y: nextBullet.y,
      dir: nextBullet.dir,
      spawn_time: nextBullet.spawn_time,
    });
  }

  #onDisconnect(id) {
    this.#sockets.delete(id);
    this.#end();
  }
}
