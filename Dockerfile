FROM rust:latest AS builder

WORKDIR /usr/src/aocbot
COPY . .
RUN cargo install --path .

FROM rust:1-alpine3.20

COPY --from=builder /usr/local/cargo/bin/aocbot /usr/local/bin/aocbot
CMD ["aocbot"]