FROM rust:latest

WORKDIR /app

COPY ./rss_feeds.db ./rss_feeds.db
COPY ./src/main ./src/main
COPY ./src/resources/config.yml ./src/resources/config.yml
COPY ./Cargo.toml .

RUN cargo build --release

CMD ["./target/release/freeflowfeeds"]
