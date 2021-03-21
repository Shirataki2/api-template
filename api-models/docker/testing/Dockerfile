FROM rust:1.50

RUN apt-get -y -q update && \
    apt-get -y -q install libpq-dev && \
    cargo install sqlx-cli --no-default-features --features postgres

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /app
 