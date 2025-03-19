ARG RUST_VERSION=1.85.0
ARG APP_NAME=big-trip
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/$APP_NAME


FROM debian:bullseye-slim AS final

RUN apt-get update && apt-get install -y curl
COPY --from=build /bin/$APP_NAME /bin/

ARG UID=1000
RUN adduser --no-create-home --uid "${UID}" app
USER app

EXPOSE 9336
CMD ["$APP_NAME"]
