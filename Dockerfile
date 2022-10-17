# build stage
FROM rust:1.64 AS builder

COPY . .

RUN cargo build --release

# release stage
FROM debian:buster-slim

COPY --from=builder ./target/release/ttr_bot ./target/release/ttr_bot

CMD ["/target/release/ttr_bot"]
