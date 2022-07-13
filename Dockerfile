# This builder builds rust source code as a statically linked library.
FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build

# Now, we need to build our _real_ Docker container, copying in only the binaries `redis-dump` and `redis-restore`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/debug/redis-* \
    /usr/local/bin/
ENV REDIS_URI=$REDIS_URI

ENTRYPOINT ["/usr/local/bin/redis-dump", "--help"]