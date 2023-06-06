FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /app

RUN export DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt -y upgrade
RUN apt-get -y install --no-install-recommends libssl1.1

COPY --from=builder /usr/local/cargo/bin/badgestore  /app/badgestore

EXPOSE 8080
CMD ["./badgestore"]