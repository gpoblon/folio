# BUILD IMAGE - from main folder : `podman build -t <image_name> -f Containerfile`
# RUN IMAGE   - from main folder : `podman run --rm -it -p 8080:8080 <image_name>`
#
# Build is split into three stages:
#   1. chef    – installs cargo-chef + the full Rust/wasm toolchain (shared base)
#   2. planner – runs `cargo chef prepare` to fingerprint the dependency tree
#   3. builder – cooks (compiles) deps from the recipe, then compiles the app
#
# As long as Cargo.toml / Cargo.lock don't change, stage 3 reuses the cached
# cooked-dependencies layer and only recompiles your own code.

# ── Stage 1 : shared toolchain base ──────────────────────────────────────────
FROM docker.io/rustlang/rust:nightly-slim AS chef

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates libssl-dev pkg-config curl \
    && rm -rf /var/lib/apt/lists/*

# wasm target required by dx bundle
RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall then use it for everything else (avoids long compiles)
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall -y \
      cargo-chef \
      dioxus-cli@0.7.3 \
      wasm-bindgen-cli@0.2.114

WORKDIR /folio

# ── Stage 2 : planner ────────────────────────────────────────────────────────
FROM chef AS planner

# Copy the full source so cargo-chef can read every Cargo.toml / Cargo.lock
COPY . .

# Produce recipe.json – a minimal description of the dependency graph.
# This file changes ONLY when dependencies change, not when src/*.rs changes.
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 3 : builder ────────────────────────────────────────────────────────
FROM chef AS builder

# Compiler flags kept identical between cook and bundle so Cargo reuses objects
ENV RUSTFLAGS="-Z unstable-options \
    -C target-feature=+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext \
    -Z threads=8 \
    -Z share-generics=y"

# Cook dependencies first – this layer is cached as long as recipe.json is unchanged
COPY --from=planner /folio/recipe.json recipe.json
RUN cargo chef cook \
      --release \
      --target wasm32-unknown-unknown \
      --recipe-path recipe.json

# Now copy the real source and build the application
COPY . .

# Bundled artefacts:
#   /folio/target/dx/app/release/web/app     – server binary
#   /folio/target/dx/app/release/web/public  – static assets + wasm
RUN dx bundle -p app --web --release

# ── Stage 4 : runtime ────────────────────────────────────────────────────────
FROM gcr.io/distroless/cc-debian13:latest-amd64 AS runtime

WORKDIR /app

COPY --from=builder /folio/target/dx/app/release/web/app    /app/server
COPY --from=builder /folio/target/dx/app/release/web/public /app/public

# Listen on all container interfaces
ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

STOPSIGNAL SIGINT

ENTRYPOINT ["/app/server"]
