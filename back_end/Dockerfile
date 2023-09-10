FROM rust:1-bookworm as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /app

RUN export DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt -y upgrade
RUN apt-get -y install --no-install-recommends libssl-dev

COPY --from=builder /usr/local/cargo/bin/badgestore  /app/badgestore

EXPOSE 8080
CMD ["./badgestore"]