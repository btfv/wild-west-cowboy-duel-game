import { EventEmitter } from 'node:events';
import { configMsg } from './config.js';

export class RoomEntry extends EventEmitter {
  connect(_id, _socket) {
    throw new Error('connect() not implemented');
  }
}

export class Lobby extends RoomEntry {
  #player;
  #onUpgrade;

  constructor(roomId, id, socket, onUpgrade) {
    super();
    this.#onUpgrade = onUpgrade;
    this.#player = { id, socket };

    socket.send(configMsg());
    socket.send({ type: 'waiting', room_id: roomId });
  }

  get player() {
    return this.#player;
  }

  connect(id, socket) {
    socket.send(configMsg());
    return this.#onUpgrade(this.#player, { id, socket });
  }
}
