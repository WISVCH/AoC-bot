FROM rust:latest AS builder

WORKDIR /usr/src/JollyFellow
COPY . .
RUN cargo install --path .

FROM rust:1-alpine3.20

COPY --from=builder /usr/local/cargo/bin/JollyFellow /usr/local/bin/JollyFellow
CMD ["JollyFellow"]