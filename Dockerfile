FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/koblas
COPY . .
RUN cargo install --path .

RUN addgroup -g 1000 -S koblas && \
    adduser -u 1000 -G koblas -S koblas

FROM scratch
COPY --from=builder /usr/local/cargo/bin/koblas /koblas
COPY --from=builder /etc/passwd /etc/passwd

USER koblas

ENV KOBLAS_ADDRESS=0.0.0.0 \
    KOBLAS_PORT=1080 \
    KOBLAS_USERS_PATH=/etc/koblas/users.toml

EXPOSE 1080

ENTRYPOINT ["/koblas"]
