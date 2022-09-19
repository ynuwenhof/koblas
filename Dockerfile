FROM rust:alpine AS builder
WORKDIR /usr/src/koblas
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/koblas /usr/local/bin/koblas
CMD ["koblas"]
