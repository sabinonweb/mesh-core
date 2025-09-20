# ---- Build stage ----
FROM rust:1.81-alpine AS builder

# Build deps + OpenSSL + Protobuf compiler
RUN apk add --no-cache \
    musl-dev build-base pkgconfig \
    openssl-dev \
    protobuf protobuf-dev

WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Real source
COPY . .

# If your build script needs an explicit path:
ENV PROTOC=/usr/bin/protoc

# Build and install to a fixed location
# (Change --bin mesh-core if your binary name differs)
RUN cargo install --path . --locked --bin mesh-core --root /out

# ---- Runtime stage ----
FROM alpine:3.19

RUN apk add --no-cache openssl ca-certificates libgcc && update-ca-certificates

# Non-root user
RUN addgroup -S app && adduser -S app -G app

WORKDIR /app

COPY --from=builder /out/bin/mesh-core /usr/local/bin/mesh-core
RUN chown app:app /usr/local/bin/mesh-core

USER app

EXPOSE 4000/udp 5000/udp 5001/udp 5002/udp
ENTRYPOINT ["mesh-core"]
