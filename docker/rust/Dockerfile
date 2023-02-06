FROM rust:1 AS base

WORKDIR /usr/src/ger_backend

RUN mkdir -p /usr/src/ger_backend/packages/backend
RUN mkdir -p /usr/src/ger_backend/packages/ger-from-row

COPY ./Cargo.toml ./Cargo.lock ./
COPY .env.local .
COPY ./packages/backend ./packages/backend
COPY ./packages/ger-from-row ./packages/ger-from-row

FROM base AS dev

RUN cargo install cargo-watch

EXPOSE 5155

CMD ["cargo", "watch", "-x", "run"]
