FROM rust:1.88.0 as builder

# Build the application
WORKDIR /performance-points-service
COPY . .
RUN cargo build --release

# Prepare runtime env
FROM alpine:latest

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
RUN apk --no-cache add libgcc

WORKDIR /home/appuser

RUN chown -R appuser:appgroup ./

COPY --from=builder /performance-points-service/target/release/performance-points-service /usr/local/bin/performance-points-service

USER appuser

ENTRYPOINT ["/usr/local/bin/performance-points-service"]