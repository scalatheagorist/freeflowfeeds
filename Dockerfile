FROM rust:latest

WORKDIR /app

COPY ./data ./data
COPY ./src/main ./src/main
COPY ./src/resources/config.yml ./src/resources/config.yml
COPY ./Cargo.toml .

RUN cargo build --release

CMD ["./target/release/freeflowfeeds"]
