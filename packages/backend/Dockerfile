FROM rust:1 AS base

WORKDIR /usr/src/ger_backend

COPY . .

FROM base AS dev

RUN cargo install cargo-watch

EXPOSE 5155

CMD ["cargo", "watch", "-x", "run"]
