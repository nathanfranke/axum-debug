# axum-debug

http server that logs a random UUID for each request and sends it to the client.

## setup

```sh
# install docker
# WARNING: insecure; use at your own risk
curl -fsSL https://get.docker.com | sudo sh
```

## deploy

```sh
# listen at http://127.0.0.1:8080
BUILDKIT_PROGRESS=plain docker compose up --build --detach
```

## debug

```sh
# setup environment
tee .env <<EOF
SERVER_ENDPOINT=127.0.0.1:8080
EOF

# listen at http://127.0.0.1:8080
cargo run
```

## example

```sh
curl -v http://127.0.0.1:8080/
# > GET / HTTP/1.1
# < HTTP/1.1 200 OK
# d0df57fc-e88f-4dee-a7ea-dc777aa4e7ad
```
