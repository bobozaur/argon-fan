FROM rust:1-slim-bullseye AS builder

ARG CASE_VERSION
ENV CASE_VERSION=$CASE_VERSION

WORKDIR /app
COPY . .

RUN cargo build --release --no-default-features --features json,${CASE_VERSION}

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/argon-fan /usr/bin/argon-fan

# Run the binary directly
CMD ["/usr/bin/argon-fan", "--config", "/data/options.json"]
