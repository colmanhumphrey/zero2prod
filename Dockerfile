FROM rust:1.47
MAINTAINER Colman Humphrey <colmanhumphrey@gmail.com>

WORKDIR app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/app"]
