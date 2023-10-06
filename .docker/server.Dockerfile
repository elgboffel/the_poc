# syntax=docker/dockerfile:1.4
FROM rust:1.71.1 AS base

ENV USER=root

WORKDIR /code

COPY . /code

RUN cargo fetch


FROM base AS development

EXPOSE 8080

CMD [ "cargo", "run", "--bin", "server", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline --bin server

FROM debian:bookworm-slim

#COPY --from=builder /code/target/release/server /server

EXPOSE 8080

CMD [ "/code/target/release/server" ]