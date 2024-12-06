FROM rust:alpine AS builder

WORKDIR /app
RUN apk update && apk upgrade
RUN apk add libc-dev openssl-dev openssl-libs-static
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

FROM alpine

WORKDIR /app
COPY --from=builder /app/target/release/JollyFellow /usr/local/bin/jollyfellow
CMD ["jollyfellow"]