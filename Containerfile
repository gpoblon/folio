# BUILD IMAGE - from main folder : `podman build -t <image_name> -f Containerfile`
# RUN IMAGE - from main folder : `podman run --rm -it <image_name>`

# 1. Build

FROM docker.io/rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates libssl-dev pkg-config curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall
ARG BINSTALL_VERSION=1.20.0
ARG BINSTALL_SHA256=4d7875788d0505547c220d2b02d3619aac0dd19b7033eba7005a8d09ff1dc433
RUN set -eux; \
    curl -fsSL \
      "https://github.com/cargo-bins/cargo-binstall/releases/download/v${BINSTALL_VERSION}/cargo-binstall-x86_64-unknown-linux-musl.tgz" \
      -o /tmp/cargo-binstall.tgz \
    && echo "${BINSTALL_SHA256}  /tmp/cargo-binstall.tgz" | sha256sum -c - \
    && tar -xzf /tmp/cargo-binstall.tgz -C /usr/local/bin cargo-binstall \
    && rm /tmp/cargo-binstall.tgz
RUN cargo binstall -y dioxus-cli@0.7.9 wasm-bindgen-cli@0.2.125

WORKDIR /folio

# Copy local code into the container
COPY . .

ENV RUSTFLAGS="-Z unstable-options -C target-feature=+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext -Z threads=8 -Z share-generics=y"
# Bundled app at: /folio/target/dx/app/release/web/server (binary file)
# Bundled app at: /folio/target/dx/app/release/web/public (folder, static assets incl. index.html and wasm binary)
RUN dx bundle -p app --web --release

# 2. Runtime

FROM gcr.io/distroless/cc-debian13:latest-amd64 AS runtime

WORKDIR /app

COPY --from=builder /folio/target/dx/app/release/web/server /app/server
COPY --from=builder /folio/target/dx/app/release/web/public /app/public

# Force the server to listen on all container interfaces
ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

STOPSIGNAL SIGINT

# Execute the renamed binary
ENTRYPOINT ["/app/server"]
