import { parse as parseUrl } from 'node:url';
import { WebSocketServer } from 'ws';
import { connect } from './registry.js';

const PORT = process.env.PORT || 3000;
const wss = new WebSocketServer({ port: PORT });

wss.on('connection', (socket, req) => {
  const { query } = parseUrl(req.url, true);

  if (!query.room || !query.id) {
    socket.close();
    return;
  }

  const rawSend = socket.send.bind(socket);
  socket.send = (msg) => rawSend(JSON.stringify(msg));
  connect(query.room, query.id, socket);
});

console.log(`ws://localhost:${PORT}`);
