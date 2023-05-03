# syntax = docker/dockerfile:1.4

FROM rust:1-slim-bullseye as builder

WORKDIR /usr/src

RUN set -eux; \
	apt update; \
	apt install -y --no-install-recommends build-essential;

RUN     USER=root cargo new fsr
WORKDIR /usr/src/fsr
COPY    .cargo .cargo
COPY    Cargo.toml Cargo.lock ./
RUN     --mount=type=cache,target=/root/.rustup \
        --mount=type=cache,target=/root/.cargo/registry \
        --mount=type=cache,target=/root/.cargo/git \
		    --mount=type=cache,target=/usr/src/target \
        cargo build --release
COPY    src src
RUN     touch src/main.rs
RUN     cargo build --release
RUN     objcopy --compress-debug-sections ./target/release/fsr ./fsr

FROM debian:bullseye-slim AS runtime 

WORKDIR /app
COPY    --from=builder /usr/src/fsr/fsr ./
COPY    static    static
COPY    templates templates

EXPOSE 8081
CMD    ["/bin/sh", "-c", "/app/fsr"]
