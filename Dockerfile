# Mise en place
FROM rust:1.47 as planner
WORKDIR app
RUN cargo install cargo-chef
# this can invalid the cache from here onwards,
# but that's fine, it's only for this stage
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cook
FROM rust:1.47 as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# Build
FROM rust:1.47 as builder
MAINTAINER Colman Humphrey <colman@slight.dev>

WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release


# Runtime stage
FROM debian:buster-slim as runtime

WORKDIR app
## Need OpenSSL for some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
