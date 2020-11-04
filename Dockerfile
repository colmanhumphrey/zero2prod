# Builder stage
FROM rust:1.47 as builder
MAINTAINER Colman Humphrey <colmanhumphrey@gmail.com>

WORKDIR app
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
COPY --from=builder /app/target/release/app ./zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
