## Builder image
FROM rust:latest AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

## Final image
FROM ubuntu:latest

WORKDIR /src

COPY --from=builder /src/target/release/gol ./

ENTRYPOINT ["/src/gol"]