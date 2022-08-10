FROM rust:latest AS builder

RUN apt update && apt install libclang-dev -y \
  && rm -rf /var/lib/apt/lists/*
RUN rustup component add rustfmt

WORKDIR /bleu-daemon

COPY . .

RUN cargo build -j4 --release --bin bleu-daemon


FROM rust:1.56-slim as runtime

WORKDIR /bleu-daemon

COPY --from=builder /bleu-daemon/target/release/bleu-daemon .

ENV RUST_LOG INFO

CMD ["./bleu-daemon", "--config-dir", "."]

EXPOSE 9999
