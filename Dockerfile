FROM rust:1.77.2 as build

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migration ./migration
COPY ./entity ./entity
COPY ./api ./api
COPY ./.env ./.env

WORKDIR /app/api

RUN cargo build --release

#FROM rust:1.77.2
#FROM alpine:3.19.1
FROM ubuntu:24.04

COPY --from=build /app/target/release/chaincue-real-estate-rust-api .
COPY ./.env ./.env

EXPOSE 8080

CMD ["./chaincue-real-estate-rust-api"]
