FROM rust:1.81-slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release -p api_server

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/api_server /app/api_server

ENV PORT=8080
ENV AISSURANCE_API_TOKEN=dev-token

EXPOSE 8080

CMD ["./api_server"]
