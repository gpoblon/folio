# BUILD IMAGE - from main folder : `podman build -t <image_name> -f Containerfile` with optional ARGs: `--build-arg PACKAGE_NAME=<binary_name>`
# RUN IMAGE - from main folder : `podman run --rm -it <image_name>`

# users can override ARGs from podman cli: `--build-arg PACKAGE_NAME=<binary_name>`
ARG PACKAGE_NAME="folio"
# replace if a specific version of rust is needed, e.g., "1.88.0"
ARG RUST_VERSION="1"

# Install required packages and tools
FROM docker.io/library/rust:${RUST_VERSION}-slim AS rust-chef
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef

WORKDIR /app

FROM rust-chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust-chef AS builder
ARG PACKAGE_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release -p ${PACKAGE_NAME}

FROM gcr.io/distroless/cc-debian13:latest-amd64 AS runtime
ARG PACKAGE_NAME
COPY --from=builder /app/target/release/${PACKAGE_NAME} /app
ENTRYPOINT ["/app"]
