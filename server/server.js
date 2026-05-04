import { parse as parseUrl } from 'node:url';
import { WebSocketServer } from 'ws';
import { getOrCreateRoom } from './room.js';

const PORT = process.env.PORT || 3000;
const wss = new WebSocketServer({ port: PORT });

wss.on('connection', (socket, req) => {
  const { query } = parseUrl(req.url, true);

  if (!query.room || !query.id) {
    socket.close();
    return;
  }

  const { room, conn } = getOrCreateRoom(query.room);

  if (room.status === 'ended') {
    socket.send(JSON.stringify({ type: 'room_ended' }));
    socket.close();
    return;
  }

  conn.add(query.id, socket);
});

console.log(`ws://localhost:${PORT}`);
