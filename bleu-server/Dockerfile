FROM rust:latest AS builder

RUN apt update && apt install libclang-dev -y \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /bleu-server

COPY . .

RUN cargo build -j4 --release --bin bleu-server


FROM rust:1.56-slim as runtime

RUN apt update && apt install libpq-dev -y \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /bleu-server
COPY --from=builder /bleu-server/target/release/bleu-server .
COPY --from=builder /bleu-server/swagger-ui ./swagger-ui

ENV RUST_LOG INFO
EXPOSE 8888

CMD ["./bleu-server"]
