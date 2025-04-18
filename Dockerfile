# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.86.0
ARG APP_NAME=task_app

################################################################################
# Stage 1: Build the Rust application
FROM rust:${RUST_VERSION}-slim as build
ARG APP_NAME

WORKDIR /app

# Install required dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        clang \
        lld \
        pkg-config \
        git \
        curl \
        ca-certificates \
        perl \
        libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy source code using bind mounts or traditional COPY (fallback if needed)
# You can use Docker buildkit bind mounts, or fall back to this:
# COPY . .

# If using Docker BuildKit with bind mounts:
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=migration,target=migration \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Stage 2: Final minimal runtime image
FROM debian:bookworm-slim as final

# Add a non-root user
ARG UID=10001
RUN useradd -u ${UID} -r -s /usr/sbin/nologin appuser

# Copy the built binary from the builder
COPY --from=build /bin/server /bin/server

USER appuser

# Expose the app's port
EXPOSE 3000

CMD ["/bin/server"]
