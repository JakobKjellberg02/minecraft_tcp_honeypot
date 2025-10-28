FROM rust:1.90.0-slim as build

WORKDIR /usr/src/minecraft-honeypot

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /usr/src/minecraft-honeypot/target/release/minecraft_tcp_honeypot /usr/local/bin/minecraft-honeypot

EXPOSE 25565

CMD [ "minecraft-honeypot" ]