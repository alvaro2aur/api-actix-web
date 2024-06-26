FROM rust:latest

WORKDIR /code

ADD Cargo.* ./

COPY src/main.rs /code/src/main.rs

RUN cargo fetch

COPY ./src /code/src

RUN cargo build --release

ENTRYPOINT ["./target/release/app"]
