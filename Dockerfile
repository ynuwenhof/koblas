FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/koblas
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/koblas /usr/local/bin/koblas

EXPOSE 1080

ENV KOBLAS_ADDRESS=0.0.0.0 \
    KOBLAS_PORT=1080 \
    KOBLAS_USERS_PATH=/etc/koblas/users.toml

ENTRYPOINT ["koblas"]
