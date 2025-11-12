################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.90.0
FROM rust:${RUST_VERSION}-slim-trixie AS build
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=core,target=core \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
apt update
apt install pkg-config libssl-dev -y
cargo build --locked --release
ls -l target
ls -l target/release
cp ./target/release/api /bin/server
EOF


FROM debian:trixie-slim AS final

RUN apt update \
 && apt install -y --no-install-recommends ca-certificates curl \
 && apt clean

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

ENTRYPOINT ["/bin/server"]
