ARG BUILD_FROM
ARG ARGON_FAN

FROM rust:latest as builder

WORKDIR /app
COPY . .

# The number of concurrent jobs is limited because lower-end systems might apparently run out of memory.
RUN cargo build --release -p $ARGON_FAN --no-default-features --features json -j 1

FROM $BUILD_FROM

COPY --from=builder /app/target/release/$ARGON_FAN /usr/bin/argon-fan

# Run the binary directly
CMD ["/usr/bin/argon-fan", "--config", "/data/options.json"]
