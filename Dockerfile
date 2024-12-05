FROM rust:latest AS builder

WORKDIR /usr/src/JollyFellow
COPY . .
RUN cargo build --release
RUN mv ./target/release/JollyFellow .

FROM rust:1-alpine3.20

COPY --from=builder /usr/src/JollyFellow/JollyFellow /usr/local/bin/JollyFellow
CMD ["/usr/local/bin/JollyFellow"]