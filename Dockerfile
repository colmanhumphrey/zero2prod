# Builder stage
FROM rust:1.47 as builder
MAINTAINER Colman Humphrey <colmanhumphrey@gmail.com>

WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release


# Runtime stage
FROM rust:1.47 as runtime

WORKDIR app
COPY --from=builder /app/target/release/app ./zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
