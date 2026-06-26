FROM node:24.15.0-alpine

RUN corepack enable

WORKDIR /app

COPY pnpm-workspace.yaml pnpm-lock.yaml package.json ./
COPY server ./server

RUN pnpm ci --filter game-server

ENV PORT=80

EXPOSE 80

CMD ["node", "server/server.js"]
