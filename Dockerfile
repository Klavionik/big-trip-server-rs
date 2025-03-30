ARG RUST_VERSION=1.85.0
FROM rust:${RUST_VERSION}-slim-bullseye AS build
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    cargo build --locked --release && \
    cp ./target/release/big-trip-server-rs /bin/big-trip


FROM debian:bullseye-slim AS final

ADD --chmod=755 https://packages.httpie.io/binaries/linux/http-latest /bin/httpie
COPY --from=build /bin/big-trip /bin/

ARG UID=1000
RUN adduser --no-create-home --uid "${UID}" app
USER app

EXPOSE 9336
CMD ["big-trip"]
