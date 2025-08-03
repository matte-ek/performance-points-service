FROM rust:1.88.0 AS builder

# Build the application
WORKDIR /performance-points-service
COPY . .
RUN cargo build --release

# Prepare runtime env
FROM ubuntu:latest

RUN useradd -r -m -g users service

WORKDIR /home/service

COPY --from=builder /performance-points-service/target/release/performance-points-service /usr/local/bin/performance-points-service

USER service

STOPSIGNAL SIGKILL

ENTRYPOINT ["/usr/local/bin/performance-points-service"]