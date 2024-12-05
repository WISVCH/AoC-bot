FROM rust:latest AS builder

WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo build --release
RUN mv ./target/release/JollyFellow ./app

FROM rust:1-alpine3.20

WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
CMD ["/usr/local/bin/app"]