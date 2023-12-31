# syntax=docker/dockerfile:1.4
FROM rust:1.71.1 AS base

ENV USER=root

WORKDIR /code

COPY . /code

RUN cargo fetch

FROM base AS development

CMD [ "cargo", "run", "--bin", "server", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline --bin server

FROM rust:1.71.1-slim AS Production

COPY --from=builder /code/target/release/server /server

CMD [ "/server" ]
