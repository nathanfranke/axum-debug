# axum-debug

http server that logs a random UUID for each request and sends it to the client.

## setup

```sh
# WARNING: INSECURE
curl -fsSL https://get.docker.com | sudo sh
```

## deploy

```sh
BUILDKIT_PROGRESS=plain docker compose up --build
# http://127.0.0.1:8080
```
