FROM --platform=linux/amd64 nigeleke/docker-dioxus-cli AS builder
WORKDIR /usr/src/app
COPY .. .
RUN dx bundle --platform=web --release

FROM nginxinc/nginx-unprivileged:alpine
WORKDIR /usr/share/nginx/html/

COPY --from=builder /usr/src/app/target/dx/scopa/release/web/public/ .
EXPOSE 8080
