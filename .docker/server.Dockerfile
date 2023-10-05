# syntax=docker/dockerfile:1.4
FROM rust:latest AS base

ENV USER=root

WORKDIR /code

COPY . /code

RUN cargo fetch


FROM base AS development

EXPOSE 8080

CMD [ "cargo", "run", "--bin", "server", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline --bin server

#FROM debian:buster-slim

EXPOSE 8080

#COPY --from=builder /code/target/release/server /server

CMD [ "/code/target/release/server" ]