FROM rust:1-buster as builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/ /app
WORKDIR /app
RUN apt-get update
RUN apt-get install -y libpq5

ENV ENV=production
CMD ["./tinker_nav_backend"]