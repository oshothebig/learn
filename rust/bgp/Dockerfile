# syntax=docker/dockerfile:1
FROM rust:1.80.1 AS builder

WORKDIR /build

RUN --mount=type=bind,target=/build,rw cargo build && cp -r target /target

FROM ubuntu:22.04 AS test

WORKDIR /work

RUN apt update
RUN apt install -y netcat-openbsd
COPY --from=builder /target target
COPY test.sh .
RUN ./test.sh