# Rust image base on alpine, AS names it as builder for later references
FROM rust:1.80-alpine AS builder

# apk is a alpine package manager, musl-dev + build-base are libraries and compilers needed to compile Rust on alpine
# openssl-dev casue we use TLS and --no-cache to avoid temporary storages
RUN apk add --no-cache musl-dev build-base openssl-dev

WORKDIR /app
# copies everything from host project to /app, rust code needs to be inside the container for compilation
COPY . .

# build it inside the container
RUN cargo build --release

# new alpine image to run Rust on it
FROM alpine:3.19

# runtime libraries to run Rust
RUN apk add --no-cache libgcc libstdc++ openssl-dev

WORKDIR /app

# Copies from relesease to app, copies the binary from builder stage to runtime
COPY --from=builder /app/target/release/mesh-core /app/mesh-core

EXPOSE 4000/udp 5000/udp 5001/udp 5002/udp

ENTRYPOINT ["/app/mesh-core"]
