FROM rust:latest as build
RUN USER=root cargo new --bin xerrar
WORKDIR /xerrar

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/xerrar*
RUN cargo build --release

FROM rust:1.60-slim-buster

COPY --from=build /xerrar/target/release/xerrar .

CMD ["./xerrar"]
