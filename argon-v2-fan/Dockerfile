ARG BUILD_FROM

FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release -p argon-v2-fan --no-default-features --features json -j 1

FROM $BUILD_FROM

COPY --from=builder /app/target/release/argon-v2-fan /usr/bin/argon-fan

# Run the binary directly
CMD ["/usr/bin/argon-fan", "--config", "/data/options.json"]
