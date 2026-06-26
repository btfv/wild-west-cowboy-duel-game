FROM node:24.15.0-alpine

RUN corepack enable

WORKDIR /app

COPY package.json pnpm-lock.yaml ./

RUN pnpm install --frozen-lockfile --prod

COPY . .

ENV PORT=80

EXPOSE 80

CMD ["node", "server.js"]
