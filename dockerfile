FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN : && \
    mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm src/main.rs && \
    :

COPY src src

RUN : && \
    touch src/main.rs && \
    cargo build --release && \
    :

FROM alpine

RUN apk add --no-cache tini

WORKDIR /app

COPY --from=builder /app/target/release/ .

EXPOSE 8080

ENTRYPOINT ["tini", "./main"]

HEALTHCHECK --interval=5s CMD sh -c "netstat -ltn | grep -q :8080"
