FROM ubuntu:20.04

RUN apt update
RUN apt install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN apt update -y
RUN apt install build-essential -y

WORKDIR /app

COPY ./src/main ./src/main
COPY ./src/resources/config.yml ./src/resources/config.yml
COPY ./Cargo.toml .

RUN cargo build --release

CMD ["./target/release/freeflowfeeds"]
