# BUILD IMAGE - from main folder : `podman build -t <image_name> -f Containerfile`
# RUN IMAGE - from main folder : `podman run --rm -it <image_name>`

# 1. Build

FROM docker.io/rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates libssl-dev pkg-config curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y dioxus-cli@0.7.3 wasm-bindgen-cli@0.2.114

WORKDIR /folio

# Copy local code into the container
COPY . .

ENV RUSTFLAGS="-Z unstable-options -C target-feature=+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext -Z threads=8 -Z share-generics=y"
# Bundled app at: /folio/target/dx/app/release/web/app (binary file)
# Bundled app at: /folio/target/dx/app/release/web/public (folder, static assets incl. index.html and wasm binary)
RUN dx bundle -p app --web --release

# 2. Runtime

FROM gcr.io/distroless/cc-debian13:latest-amd64 AS runtime

WORKDIR /app

COPY --from=builder /folio/target/dx/app/release/web/app /app/server
COPY --from=builder /folio/target/dx/app/release/web/public /app/public

# Force the server to listen on all container interfaces
ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

STOPSIGNAL SIGINT

# Execute the renamed binary
ENTRYPOINT ["/app/server"]
