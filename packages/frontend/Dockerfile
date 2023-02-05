FROM node:lts

WORKDIR /usr/src/ger_frontend

RUN npm install -g pnpm

COPY package.json ../../.npmrc ../../pnpm-lock.yaml ./

RUN pnpm fetch --prod

COPY . .
RUN pnpm install -r --prod

EXPOSE 3000
CMD ["pnpm", "dev"]
