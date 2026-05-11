import { Lobby } from './lobby.js';
import { Room } from './room.js';

const rooms = new Map();

export function connect(roomId, id, socket) {
  const entry = rooms.get(roomId);

  if (!entry) {
    const lobby = new Lobby(roomId, id, socket, (p1, p2) => {
      const room = new Room(p1, p2);
      room.on('ended', () => rooms.delete(roomId));

      return room;
    });

    rooms.set(roomId, lobby);
    return;
  }

  rooms.set(roomId, entry.connect(id, socket));
}
