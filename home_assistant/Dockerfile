FROM rust:latest AS builder

ARG ARGON_FAN
ENV ARGON_FAN=$ARGON_FAN

WORKDIR /app
COPY . .

RUN cargo build --release -p ${ARGON_FAN} --no-default-features --features json

FROM debian:bullseye-slim

ARG ARGON_FAN

COPY --from=builder /app/target/release/${ARGON_FAN} /usr/bin/argon-fan

# Run the binary directly
CMD ["/usr/bin/argon-fan", "--config", "/data/options.json"]
